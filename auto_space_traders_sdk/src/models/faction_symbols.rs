/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// FactionSymbols : The symbol of the faction.

/// The symbol of the faction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FactionSymbols {
    #[serde(rename = "COSMIC")]
    Cosmic,
    #[serde(rename = "VOID")]
    Void,
    #[serde(rename = "GALACTIC")]
    Galactic,
    #[serde(rename = "QUANTUM")]
    Quantum,
    #[serde(rename = "DOMINION")]
    Dominion,
    #[serde(rename = "ASTRO")]
    Astro,
    #[serde(rename = "CORSAIRS")]
    Corsairs,
    #[serde(rename = "OBSIDIAN")]
    Obsidian,
    #[serde(rename = "AEGIS")]
    Aegis,
    #[serde(rename = "UNITED")]
    United,
    #[serde(rename = "SOLITARY")]
    Solitary,
    #[serde(rename = "COBALT")]
    Cobalt,
    #[serde(rename = "OMEGA")]
    Omega,
    #[serde(rename = "ECHO")]
    Echo,
    #[serde(rename = "LORDS")]
    Lords,
    #[serde(rename = "CULT")]
    Cult,
    #[serde(rename = "ANCIENTS")]
    Ancients,
    #[serde(rename = "SHADOW")]
    Shadow,
    #[serde(rename = "ETHEREAL")]
    Ethereal,

}

impl ToString for FactionSymbols {
    fn to_string(&self) -> String {
        match self {
            Self::Cosmic => String::from("COSMIC"),
            Self::Void => String::from("VOID"),
            Self::Galactic => String::from("GALACTIC"),
            Self::Quantum => String::from("QUANTUM"),
            Self::Dominion => String::from("DOMINION"),
            Self::Astro => String::from("ASTRO"),
            Self::Corsairs => String::from("CORSAIRS"),
            Self::Obsidian => String::from("OBSIDIAN"),
            Self::Aegis => String::from("AEGIS"),
            Self::United => String::from("UNITED"),
            Self::Solitary => String::from("SOLITARY"),
            Self::Cobalt => String::from("COBALT"),
            Self::Omega => String::from("OMEGA"),
            Self::Echo => String::from("ECHO"),
            Self::Lords => String::from("LORDS"),
            Self::Cult => String::from("CULT"),
            Self::Ancients => String::from("ANCIENTS"),
            Self::Shadow => String::from("SHADOW"),
            Self::Ethereal => String::from("ETHEREAL"),
        }
    }
}

impl Default for FactionSymbols {
    fn default() -> FactionSymbols {
        Self::Cosmic
    }
}




