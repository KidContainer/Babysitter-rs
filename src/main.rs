use ::log::{error, info, logger, trace, warn, LevelFilter};
use std::io::Write;
use tokio::net::TcpListener;

mod cellar;
mod socket_reader;

macro_rules! log_init {
    () => {
        env_logger::Builder::from_default_env()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{}:{} {} [{}] - {}",
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                    record.level(),
                    record.args()
                )
            })
            .init();
    };
}

#[allow(unreachable_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log_init!();
    info!("welcome to babysitter");
    let ip: String = match std::env::var("PORT") {
        Ok(ip) => ip,
        Err(e) => {
            warn!("use default port number, which is 12345");
            "12345".to_string()
        }
    };
    let listener = TcpListener::bind(format!("0.0.0.0:{}", ip)).await?;

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("{} has connected in", addr.to_string());
        tokio::spawn(async move {

        });
    }
    error!("something wrong with the service, please restart");

    Ok(())
}

