use anyhow::Result;
use gtfs_structures::Gtfs;
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, field::MakeExt};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .map_fmt_fields(|f| f.debug_alt())
        .init();

    info!("Starting");

    let gtfs = Gtfs::new("./SEQ_GTFS.zip")?;

    debug!(stops=?gtfs.stops);

    info!("Done");

    Ok(())
}
