#![allow(dead_code)]
#![doc = include_str!("../README.md")]

pub mod utilities;
pub mod errors;
pub mod common;
pub mod headers;
pub mod tables;

mod segment;
mod section;
mod binary;

pub use segment::Segment;
pub use section::Section;
pub use binary::Binary;