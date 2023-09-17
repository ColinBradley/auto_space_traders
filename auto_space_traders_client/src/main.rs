#![feature(extract_if)]
#![feature(async_closure)]

use std::collections::{HashMap, HashSet};

use auto_space_traders_sdk::{
    apis::{agents_api, configuration::Configuration, contracts_api, fleet_api, systems_api},
    models::{waypoint_trait, TradeSymbol},
};
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

    let account_symbol = agent.symbol;
    println!("Hello, {account_symbol}!");

    println!("Getting contracts...");
    let contracts_result = contracts_api::get_contracts(&space_configuration, None, None).await?;

    let mut contracts = contracts_result.data;

    for contract in contracts.extract_if(|c| !c.accepted).collect::<Vec<_>>() {
        println!("Accepting contract: {contract:?}");

        let accept_contract_result =
            contracts_api::accept_contract(&space_configuration, &contract.id).await?;

        agent = accept_contract_result.data.agent;
        contracts.push(*accept_contract_result.data.contract);
    }

    println!("Getting ships...");
    let mut ships = fleet_api::get_my_ships(&space_configuration, None, None)
        .await?
        .data;

    let space_config_ref = &space_configuration;
    let mut unique_systems = ships
        .iter()
        .map(|s| s.nav.system_symbol.clone())
        .collect::<HashSet<_>>();
    let systems_futures = unique_systems.drain().map(async move |system| {
        let data = systems_api::get_system_waypoints(space_config_ref, &system, None, None)
            .await
            .expect("Unable to get system waypoints")
            .data;

        (system, data)
    });

    let waypoints_by_system = join_all(systems_futures)
        .await
        .into_iter()
        .collect::<HashMap<_, _>>();

    let contract_mining_jobs = if let Some(contract_item_to_get) =
        contracts.get(0).and_then(|first_contract| {
            first_contract
                .terms
                .deliver
                .as_ref()
                .and_then(|deliveries| deliveries.get(0).map(|first| &first.trade_symbol))
        }) {
        matches!(
            contract_item_to_get,
            TradeSymbol::IronOre
                | TradeSymbol::CopperOre
                | TradeSymbol::AluminumOre
                | TradeSymbol::SilverOre
                | TradeSymbol::GoldOre
                | TradeSymbol::PlatinumOre
                | TradeSymbol::UraniteOre
                | TradeSymbol::MeritiumOre
        )
        .then(|| {
            let mineable_waypoints = waypoints_by_system
                .values()
                .flatten()
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
                .collect::<Vec<_>>();

            if mineable_waypoints.is_empty() {
                None
            } else {
                Some((*contract_item_to_get, mineable_waypoints))
            }
        })
        .unwrap_or(None)
    } else {
        None
    };

    let cooldowns_by_ship_symbol =
        join_all(ships.iter().map(async move |ship| {
            fleet_api::get_ship_cooldown(space_config_ref, &ship.symbol).await
        }))
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
        .collect::<HashMap<_, _>>();

    for ship in ships.iter() {
        let is_full_ish = ship.cargo.capacity - ship.cargo.units < 3;
        let active_cooldown = cooldowns_by_ship_symbol.get(&ship.symbol);
        let has_contract_item = match contract_mining_jobs {
            Some((symbol, _)) => ship
                .cargo
                .inventory
                .iter()
                .any(|item| item.symbol == symbol),
            None => false,
        };

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
