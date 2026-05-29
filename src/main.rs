use clap::Parser;
use log::LevelFilter;

use crate::{args::MigraineArgs, logger::SimpleLogger, migraine::Migraine};

mod algorithm;
mod args;
mod color;
mod downsample;
mod error;
mod io;
mod migraine;
mod scale;
mod visualizer;
mod logger;

static LOGGER: SimpleLogger = SimpleLogger;

fn main() -> std::io::Result<()> {
    let args = MigraineArgs::parse();

    let log_level = match args.verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log_level))
        .expect("Failed to initialize logger");

    Migraine::restore(
        args.path,
        args.scale,
        args.width,
        args.height,
        args.colors,
        args.max_colors,
    )?;
    
    Ok(())
}
