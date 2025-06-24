//! GTFS
//!
//! This module handles a few responsibilities:
//! - Loading static gtfs data via gtfs-structures.
//! - Loading real time gtfs data via protobufs.
//! - Cleaning that up and verifying it.
mod static_gtfs;

use std::collections::HashMap;

use crate::transit_realtime::FeedMessage;
use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use chrono::DateTime;
use chrono::FixedOffset;
use futures::future::try_join_all;
use gtfs_structures::Gtfs;
use gtfs_structures::RawGtfs;
use prost::Message;
use reqwest::{Client, header::LAST_MODIFIED};
use tokio::task::spawn_blocking;
use tracing::{info, instrument};

/// Static GTFS wrapper.
pub struct StaticGtfs(pub RawGtfs);

/// Realtime GTFS wrapper. Stores a vec of FeedMessages, as there may be multiple.
pub struct RealtimeGtfs(pub Vec<FeedMessage>);

/// Checks the last-modified header for a url and returns it, or None if the header is not present.
#[instrument]
pub async fn last_modified(url: String, client: &Client) -> Result<Option<DateTime<FixedOffset>>> {
    let response = client.head(url).send().await?;

    let Some(last_modified) = response.headers().get(LAST_MODIFIED) else {
        return Ok(None);
    };
    let last_modified_str = last_modified.to_str()?;
    Ok(Some(DateTime::parse_from_rfc2822(last_modified_str)?))
}

/// Loads a static gtfs feed from the given path, which is either a file or url.
/// With the translink dataset this can take quite a while (~40 seconds on my pc).
#[instrument]
pub async fn load_static_gtfs(url: String) -> Result<StaticGtfs> {
    info!("Loading static GTFS. This may take a while.");
    let gtfs = spawn_blocking(move || RawGtfs::new(&url)).await??;
    info!("Finished loading static GTFS");
    Ok(StaticGtfs(gtfs))
}

/// Loads realtime gtfs updates.
/// Takes a vec of urls as translink dont have one unified feed.
#[instrument]
pub async fn load_realtime_gtfs(urls: Vec<String>) -> Result<RealtimeGtfs> {
    info!("Loading realtime GTFS.");
    let futures = urls.into_iter().map(|url| async {
        let pb = reqwest::get(url).await?.bytes().await?;
        FeedMessage::decode(pb).context("Failed to decode")
    });

    let messages = try_join_all(futures).await?;
    info!("Finished loading realtime GTFS.");
    Ok(RealtimeGtfs(messages))
}
