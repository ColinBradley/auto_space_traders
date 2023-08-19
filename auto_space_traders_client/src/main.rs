use auto_space_traders_sdk::apis::{agents_api, configuration::Configuration};
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

    let initial_agent_result = agents_api::get_my_agent(&space_configuration)
        .await
        .expect("Failed to get my agent");

    let account_symbol = initial_agent_result.data.symbol;
    println!("Hello, {account_symbol}!");

    Ok(())
}
