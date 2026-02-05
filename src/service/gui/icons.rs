// stores all icon data

use iced::Font;

pub const ICON_FONT: Font = Font::with_name("peanuticons");
pub struct IconChar(pub &'static str);

pub const PAUSE: IconChar = IconChar("\u{e908}");
pub const STOP_DOWNLOAD: IconChar = IconChar("\u{e901}");
pub const START_DOWNLOAD: IconChar = IconChar("\u{e902}");
pub const PENDING: IconChar = IconChar("\u{e903}");
pub const SKIP: IconChar = IconChar("\u{e904}");
pub const ORGANIZE: IconChar = IconChar("\u{e905}");
pub const SHUFFLE: IconChar = IconChar("\u{e906}");
pub const PREVIOUS: IconChar = IconChar("\u{e907}");
pub const PLAY: IconChar = IconChar("\u{e900}");
pub const LEFT_ARROW: IconChar = IconChar("\u{ea40}");
pub const VOLUME_HIGH: IconChar = IconChar("\u{ea26}");
pub const VOLUME_MEDIUM: IconChar = IconChar("\u{ea27}");
pub const VOLUME_LOW: IconChar = IconChar("\u{ea28}");
pub const VOLUME_MUTE: IconChar = IconChar("\u{ea2a}");
pub const LOOP: IconChar = IconChar("\u{ea2e}");
