#![feature(extract_if)]
#![feature(async_closure)]
#![feature(let_chains)]

use std::collections::{HashMap, HashSet};

use auto_space_traders_sdk::{
    apis::{
        agents_api,
        configuration::Configuration,
        contracts_api::{self, AcceptContractError},
        fleet_api, systems_api, Error,
    },
    models::{waypoint_trait, Agent, Contract, Ship, TradeSymbol, Waypoint},
};
use chrono::{DateTime, FixedOffset};
use futures::future::join_all;
use serde::Deserialize;

#[derive(Deserialize)]
struct ApplicationConfiguration {
    access_token: String,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let application_config: ApplicationConfiguration = serde_json::from_str(
        &tokio::fs::read_to_string("config.json")
            .await
            .expect("Could not find/load config.json"),
    )?;

    let space_configuration = Configuration {
        bearer_access_token: Some(application_config.access_token),
        ..Default::default()
    };

    let mut agent = agents_api::get_my_agent(&space_configuration).await?.data;

    let account_symbol = agent.symbol.clone();
    println!("Hello, {account_symbol}!");

    println!("Getting contracts...");
    let mut contracts = contracts_api::get_contracts(&space_configuration, None, None)
        .await?
        .data;

    accept_all_contracts(&space_configuration, &mut contracts, &mut agent).await?;

    println!("Getting ships...");
    let mut ships = fleet_api::get_my_ships(&space_configuration, None, None)
        .await?
        .data;

    let waypoints_by_system = get_waypoints_by_system(&space_configuration, &ships).await;

    let contract_goods_to_get = get_trade_goods_from_contracts(&contracts);

    let (minable_contract_items, minable_waypoints) = get_mineable_trade_items_and_waypoints(
        &contract_goods_to_get,
        &waypoints_by_system.values().flatten().collect::<Vec<_>>(),
    );

    let cooldowns_by_ship_symbol = get_cooldowns_by_ship_symbol(&ships, &space_configuration).await;

    for ship in ships.iter() {
        let is_full_ish = ship.cargo.capacity - ship.cargo.units < 3;
        let active_cooldown = cooldowns_by_ship_symbol.get(&ship.symbol);
        let has_contract_item = ship
            .cargo
            .inventory
            .iter()
            .any(|item| minable_contract_items.contains(&&item.symbol));

        let has_junk = match ship.cargo.inventory.len() {
            0 => false,
            1 => !has_contract_item,
            _ => true,
        };

        if is_full_ish {
            if has_junk {
                println!("Sell junk");
            } else {
                println!("Turn in items");
            }
        } else if active_cooldown.is_some() {
            if has_junk {
                println!("Sell junk");
            } else {
                println!("Idle");
            }
        } else {
            println!("Mine");
        }
    }

    Ok(())
}

async fn accept_all_contracts(
    space_configuration: &Configuration,
    contracts: &mut Vec<Contract>,
    agent: &mut Box<Agent>,
) -> Result<(), Error<AcceptContractError>> {
    for contract in contracts.extract_if(|c| !c.accepted).collect::<Vec<_>>() {
        println!("Accepting contract: {contract:?}");

        let accept_contract_result =
            contracts_api::accept_contract(space_configuration, &contract.id).await?;

        *agent = accept_contract_result.data.agent;
        contracts.push(*accept_contract_result.data.contract);
    }

    Ok(())
}

async fn get_waypoints_by_system(
    space_configuration: &Configuration,
    ships: &[Ship],
) -> HashMap<String, Vec<Waypoint>> {
    let mut unique_systems = ships
        .iter()
        .map(|s| s.nav.system_symbol.clone())
        .collect::<HashSet<_>>();

    // let space_configuration = &space_configuration;
    let systems_futures = unique_systems.drain().map(async move |system| {
        let data = systems_api::get_system_waypoints(space_configuration, &system, None, None)
            .await
            .expect("Unable to get system waypoints")
            .data;

        (system, data)
    });

    join_all(systems_futures).await.into_iter().collect()
}

fn get_trade_goods_from_contracts(contracts: &[Contract]) -> Vec<TradeSymbol> {
    contracts
        .iter()
        .flat_map(|contract| {
            contract
                .terms
                .deliver
                .as_ref()
                .map_or_else(Vec::new, |deliveries| {
                    deliveries
                        .iter()
                        .map(|delivery| delivery.trade_symbol)
                        .collect()
                })
        })
        .collect()
}

fn is_trade_symbol_ore(symbol: &TradeSymbol) -> bool {
    matches!(
        symbol,
        TradeSymbol::IronOre
            | TradeSymbol::CopperOre
            | TradeSymbol::AluminumOre
            | TradeSymbol::SilverOre
            | TradeSymbol::GoldOre
            | TradeSymbol::PlatinumOre
            | TradeSymbol::UraniteOre
            | TradeSymbol::MeritiumOre
    )
}

fn get_mineable_trade_items_and_waypoints<'t, 'w>(
    trade_items: &'t [TradeSymbol],
    waypoints: &[&'w Waypoint],
) -> (Vec<&'t TradeSymbol>, Vec<&'w Waypoint>) {
    (
        trade_items
            .iter()
            .filter(|t| is_trade_symbol_ore(t))
            .collect(),
        get_mineable_waypoints(waypoints),
    )
}

fn get_mineable_waypoints<'w>(waypoints: &[&'w Waypoint]) -> Vec<&'w Waypoint> {
    waypoints
        .iter()
        .filter(|w| {
            w.traits.iter().any(|t| {
                matches!(
                    t.symbol,
                    waypoint_trait::Symbol::MineralDeposits
                        | waypoint_trait::Symbol::RareMetalDeposits
                        | waypoint_trait::Symbol::CommonMetalDeposits
                        | waypoint_trait::Symbol::PreciousMetalDeposits
                )
            })
        })
        .copied()
        .collect()
}

async fn get_cooldowns_by_ship_symbol(
    ships: &[Ship],
    configuration: &Configuration,
) -> HashMap<String, DateTime<FixedOffset>> {
    join_all(
        ships
            .iter()
            .map(async move |ship| fleet_api::get_ship_cooldown(configuration, &ship.symbol).await),
    )
    .await
    .drain(..)
    .filter_map(|c| {
        c.map(|c| {
            (
                c.data.ship_symbol,
                chrono::DateTime::parse_from_rfc3339(&c.data.expiration).unwrap(),
            )
        })
        .ok()
    })
    .collect()
}
