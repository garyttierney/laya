use std::path::Path;

use clap::Parser;
use laya::runtime::tokio::TokioServerRuntime;
use telemetry::install_telemetry;
use tracing::{info, info_span};

#[path = "laya-server/telemetry.rs"]
mod telemetry;

#[derive(clap::ValueEnum, Clone, Default, Debug)]
pub enum Rt {
    #[cfg(all(feature = "glommio", target_os = "linux"))]
    Glommio,
    #[cfg(feature = "rt-tokio")]
    #[default]
    Tokio,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = install_telemetry() {
        eprint!("Failed to install telemetry ({e}). Logging will be unavailable");
    }

    info!(options = ?args, "begin startup");

    match args.runtime {
        #[cfg(all(feature = "glommio", target_os = "linux"))]
        Rt::Glommio => {
            todo!()
        }

        #[cfg(feature = "rt-tokio")]
        Rt::Tokio => laya::start::<TokioServerRuntime>(()),
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    #[arg(short, long, default_value = "./share/")]
    path: Box<Path>,

    #[arg(short, long, default_value = "tokio")]
    runtime: Rt,
}
