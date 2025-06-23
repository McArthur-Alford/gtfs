pub mod db;
pub mod vars;

use anyhow::Result;
use prost::Message;
use std::time::Duration;
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

    let gtfs = load_gtfs("./seq_gtfs.zip".to_owned()).await.unwrap();

    // TODO:
    // Pull out a db module and connect with sqlx.
    // Save the Gtfs struct above into it (skipping things that already exist).
    //
    // Then switch over to a periodic task that pulls from the translink url.

    gtfs.print_stats();
    return Ok(());

    loop {
        if let Err(e) = poll().await {
            error!(e=?e);
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn load_gtfs(url: String) -> Result<gtfs_structures::Gtfs> {
    let gtfs = gtfs_structures::Gtfs::new(&url)?;
    Ok(gtfs)
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
