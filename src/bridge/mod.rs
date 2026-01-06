//! BRIDGE
//!
//! Handles briding data from gtfs-structures and protobufs (for realtime)
//! to the DB types defined in db/types.rs

pub mod realtime_bridge;
pub mod static_bridge;
