//! BRIDGE
//!
//! Handles briding data from gtfs-structures and protobufs (for realtime)
//! to the DB types defined in db/types.rs

mod realtime_bridge;
mod static_bridge;

pub use static_bridge::ToDB;
