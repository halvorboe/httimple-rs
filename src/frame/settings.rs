
use frame::{Size};

/// The default value of SETTINGS_HEADER_TABLE_SIZE
pub const DEFAULT_SETTINGS_HEADER_TABLE_SIZE: usize = 4_096;

/// The default value of SETTINGS_INITIAL_WINDOW_SIZE
pub const DEFAULT_INITIAL_WINDOW_SIZE: u32 = 65_535;

/// The default value of MAX_FRAME_SIZE
pub const DEFAULT_MAX_FRAME_SIZE: Size = 16_384;

/// INITIAL_WINDOW_SIZE upper bound
pub const MAX_INITIAL_WINDOW_SIZE: usize = (1 << 31) - 1;

/// MAX_FRAME_SIZE upper bound
pub const MAX_MAX_FRAME_SIZE: Size = (1 << 24) - 1;


pub struct Settings {
    
}