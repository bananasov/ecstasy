use log::{error, info, warn};

use crate::collector::CollectorCore;
use crate::error::EcstasyError;
use crate::logger::EcstasyLogger;
use crate::params::EcstasyParams;

mod collector;
mod collectors;
mod error;
mod item;
mod logger;
mod manifest;
mod params;
mod stats;
mod utility;

fn main() -> Result<(), EcstasyError> {
    let params = EcstasyParams::new()?;
    EcstasyLogger::init(params.verbose)?;
    if params.insane {
        warn!("Insanity mode enabled! I really hope you know what you're doing...");
    }
    if params.tags.is_empty() && !params.insane {
        error!(
            "{} {}",
            "Leaving the tags empty will try to rip every single image from a given source.",
            "If you are absolutely sure you want to do this, add the \"--insanity\" argument."
        );
    } else {
        let mut collector = CollectorCore::new(params.clone());
        let items = collector.collect();
        if !params.debug {
            collector.download(Some(items.clone()))?;
        } else {
            info!("Skipped downloading phase, debugging mode is enabled.");
        }
        info!("Downloaded {} items with tags: ({})", items.len(), params.tags.join(","));
        info!("All jobs finished, goodbye!");
    }
    Ok(())
}
