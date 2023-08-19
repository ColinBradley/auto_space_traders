/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct System {
    /// The symbol of the system.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// The symbol of the sector.
    #[serde(rename = "sectorSymbol")]
    pub sector_symbol: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::SystemType,
    /// Position in the universe in the x axis.
    #[serde(rename = "x")]
    pub x: i32,
    /// Position in the universe in the y axis.
    #[serde(rename = "y")]
    pub y: i32,
    /// Waypoints in this system.
    #[serde(rename = "waypoints")]
    pub waypoints: Vec<crate::models::SystemWaypoint>,
    /// Factions that control this system.
    #[serde(rename = "factions")]
    pub factions: Vec<crate::models::SystemFaction>,
}

impl System {
    pub fn new(symbol: String, sector_symbol: String, r#type: crate::models::SystemType, x: i32, y: i32, waypoints: Vec<crate::models::SystemWaypoint>, factions: Vec<crate::models::SystemFaction>) -> System {
        System {
            symbol,
            sector_symbol,
            r#type,
            x,
            y,
            waypoints,
            factions,
        }
    }
}


