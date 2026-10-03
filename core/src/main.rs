use anyhow::Result;
use midas_core::MidasRuntime;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string()),
        )
        .init();

    let mut runtime = MidasRuntime::new();

    runtime.start()?;

    info!("{}", runtime.identity.summary());

    let mut interval =
        tokio::time::interval(
            std::time::Duration::from_secs(5),
        );

    loop {
        interval.tick().await;

        runtime.heartbeat()?;
    }
}
