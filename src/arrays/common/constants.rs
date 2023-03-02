use crate::headers::common::ranges::Ranges;

// Ranges for array items
pub const ADDRESS: Ranges = Ranges::new(0x00..0x04,0x00..0x08); // i32, i64