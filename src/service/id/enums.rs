use strum_macros::{Display, EnumString};

#[derive(Debug, EnumString, Display)]
pub enum Platform {
    #[strum(serialize = "yt")]
    Youtube,
}
