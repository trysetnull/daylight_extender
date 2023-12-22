use clap::Parser;
use clap_num::number_range;
use log::{info, LevelFilter};
use shelly::api::Gen2DeviceClient;
use simple_logger::SimpleLogger;

fn range_0_24(s: &str) -> Result<u8, String> {
    number_range(s, 0, 24)
}

/// Extend daylight to a give total time in hours
/// by switching on a light switch controlled by
/// a smart relay.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// IP address of a Gen 2 Shelly Device.
    #[arg(long, default_value = "192.168.0.232")]
    host: String,

    /// Total day length in hours (0 -- 24).
    #[arg(long, default_value_t = 12, value_parser=range_0_24)]
    total_day_length: u8,

    /// Make the operation more talkative.
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Silent mode.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    silent: bool,
}

impl Cli {
    fn log_level(&self) -> LevelFilter {
        if self.silent {
            return LevelFilter::Off;
        }
        match self.verbose {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    SimpleLogger::new()
        .with_level(cli.log_level())
        .init()
        .unwrap();

    let client = Gen2DeviceClient::new(&cli.host);
    let core = daylight_extender::Controller::new(&client);
    let revision = core.execute(cli.total_day_length).await?;
    info!("SUCCESS: Schedule (Rev: {revision}) to extend day length created or updated!");
    Ok(())
}
