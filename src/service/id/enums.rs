use strum_macros::{Display, EnumString};

#[derive(Debug, EnumString, Display, PartialEq, Eq, Hash, Clone)]
pub enum Platform {
    #[strum(serialize = "yt")]
    Youtube,
}
