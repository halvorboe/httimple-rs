mod settings;

use self::settings::Settings;

pub type Size = u32;

pub enum Frame {
    Settings(Settings),
}