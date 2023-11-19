/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// Survey : A resource survey of a waypoint, detailing a specific extraction location and the types of resources that can be found there.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Survey {
    /// A unique signature for the location of this survey. This signature is verified when attempting an extraction using this survey.
    #[serde(rename = "signature")]
    pub signature: String,
    /// The symbol of the waypoint that this survey is for.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// A list of deposits that can be found at this location. A ship will extract one of these deposits when using this survey in an extraction request. If multiple deposits of the same type are present, the chance of extracting that deposit is increased.
    #[serde(rename = "deposits")]
    pub deposits: Vec<crate::models::SurveyDeposit>,
    /// The date and time when the survey expires. After this date and time, the survey will no longer be available for extraction.
    #[serde(rename = "expiration")]
    pub expiration: String,
    /// The size of the deposit. This value indicates how much can be extracted from the survey before it is exhausted.
    #[serde(rename = "size")]
    pub size: Size,
}

impl Survey {
    /// A resource survey of a waypoint, detailing a specific extraction location and the types of resources that can be found there.
    pub fn new(
        signature: String,
        symbol: String,
        deposits: Vec<crate::models::SurveyDeposit>,
        expiration: String,
        size: Size,
    ) -> Survey {
        Survey {
            signature,
            symbol,
            deposits,
            expiration,
            size,
        }
    }
}

/// The size of the deposit. This value indicates how much can be extracted from the survey before it is exhausted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Size {
    #[serde(rename = "SMALL")]
    Small,
    #[serde(rename = "MODERATE")]
    Moderate,
    #[serde(rename = "LARGE")]
    Large,
}

impl Default for Size {
    fn default() -> Size {
        Self::Small
    }
}
