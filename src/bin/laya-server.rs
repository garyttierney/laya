use std::path::Path;
use std::time::Duration;

use clap::Parser;
use laya::runtime::tokio::TokioRuntime;
use telemetry::install_telemetry_collector;
use tracing::info;

#[path = "laya-server/telemetry.rs"]
mod telemetry;

#[derive(clap::ValueEnum, Clone, Default, Debug)]
pub enum Rt {
    #[cfg(all(feature = "rt-glommio", target_os = "linux"))]
    Glommio,
    #[cfg(feature = "rt-tokio")]
    #[default]
    Tokio,
}

fn main() {
    let args = Args::parse();
    let telemetry_rt = install_telemetry_collector();

    info!(options = ?args, "begin startup");

    match args.runtime {
        #[cfg(all(feature = "rt-glommio", target_os = "linux"))]
        Rt::Glommio => {
            todo!()
        }

        #[cfg(feature = "rt-tokio")]
        Rt::Tokio => laya::start::<TokioRuntime>(()),
    }

    telemetry_rt.shutdown_timeout(Duration::from_secs(5));
}

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    #[arg(short, long, default_value = "./share/")]
    path: Box<Path>,

    #[arg(short, long, default_value = "tokio")]
    runtime: Rt,
}
