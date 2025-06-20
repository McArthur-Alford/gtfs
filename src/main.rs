use std::time::Duration;

use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime};
use prost::Message;
use tracing::{debug, error, info};
use tracing_subscriber::{EnvFilter, field::MakeExt};

pub mod transit_realtime {
    include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .map_fmt_fields(|f| f.debug_alt())
        .init();

    info!("Starting");

    loop {
        if let Err(e) = poll().await {
            error!(e=?e);
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn poll() -> Result<()> {
    let pb = reqwest::get("https://gtfsrt.api.translink.com.au/api/realtime/SEQ/TripUpdates")
        .await?
        .bytes()
        .await?;

    let message = transit_realtime::FeedMessage::decode(pb)?;

    // let time = message.header.timestamp() as i64;
    // let time = DateTime::from_timestamp(time, 0);

    // debug!(header=?message.header);
    // debug!(time=?time);

    for entity in message.entity {
        let Some(trip) = entity.trip_update.as_ref() else {
            continue;
        };

        debug!(trip=?trip);
    }

    Ok(())
}
