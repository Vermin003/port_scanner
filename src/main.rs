use clap::Parser;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio::sync::Semaphore;
use std::sync::Arc;
use tokio::task::JoinSet;

#[derive(Clone)]
enum PortInput {
    Single(u16),
    Range(u16, u16),
}

impl std::str::FromStr for PortInput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split_once('-') {
            let start = start.parse::<u16>()?;
            let end = end.parse::<u16>()?;
            Ok(PortInput::Range(start, end))
        } else {
            Ok(PortInput::Single(s.parse::<u16>()?))
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    // Host to attempt port scan on
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[arg(short, long)]
    ports: PortInput,

    #[arg(short, long, default_value_t = 500)]
    timeout: u64,

    #[arg(long, default_value_t = false)]
    show_closed: bool,

    #[arg(short, long, default_value_t = 500)]
    max_concurrent: usize,
}

async fn scan_port(host: &str, port: u16, duration: u64) -> bool {
    let addr = format!("{}:{}", host, port);
    let duration = Duration::from_millis(duration);

    match timeout(duration, TcpStream::connect(&addr)).await {
        Ok(Ok(_)) => true,
        _ => false,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.ports {
        PortInput::Single(port) => {
            let open = scan_port(&args.host, port, args.timeout).await;
            if open {
                println!("{}:{} is open", args.host, port);
            } else {
                println!("{}:{} is closed", args.host, port);
            }
        }
        PortInput::Range(start, end) => {
            let semaphore = Arc::new(Semaphore::new(args.max_concurrent));
            let mut set = JoinSet::new();

            for port in start..=end {
                let permit = semaphore.clone().acquire_owned().await;
                let host = args.host.clone();
                let timeout = args.timeout;

                set.spawn(async move {
                    let _permit = permit;
                    let open = scan_port(&host, port, timeout).await;
                    (port, open)
                });
            }

            let mut results = set.join_all().await;
            results.sort_by_key(|(port, _)| *port);

            println!("\nScan results for {}:", args.host);
            for (port, open) in results {
                match open {
                    true => {
                        println!("{}:{} is open", args.host, port);
                    }
                    false if args.show_closed => {
                        println!("{}:{} is closed", args.host, port);
                    },
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
