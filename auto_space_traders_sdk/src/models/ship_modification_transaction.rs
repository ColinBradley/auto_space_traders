/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// ShipModificationTransaction : Result of a transaction for a ship modification, such as installing a mount or a module.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipModificationTransaction {
    /// The symbol of the waypoint where the transaction took place.
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
    /// The symbol of the ship that made the transaction.
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    /// The symbol of the trade good.
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    /// The total price of the transaction.
    #[serde(rename = "totalPrice")]
    pub total_price: u32,
    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

impl ShipModificationTransaction {
    /// Result of a transaction for a ship modification, such as installing a mount or a module.
    pub fn new(
        waypoint_symbol: String,
        ship_symbol: String,
        trade_symbol: String,
        total_price: u32,
        timestamp: String,
    ) -> ShipModificationTransaction {
        ShipModificationTransaction {
            waypoint_symbol,
            ship_symbol,
            trade_symbol,
            total_price,
            timestamp,
        }
    }
}
