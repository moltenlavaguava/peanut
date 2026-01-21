use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, EnumString, Display, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub enum Platform {
    #[strum(serialize = "yt")]
    Youtube,
    #[strum(serialize = "mb")]
    MusicBrainz,
}
