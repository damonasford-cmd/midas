use anyhow::Result;
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use midas_core::{
    config::MidasConfig,
    core::MidasCore,
    identity::IdentityState,
    perception::Perception,
};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    info!("========================================");
    info!("MIDAS");
    info!("Aeron Asford");
    info!("========================================");
    info!("Initialisation du Core...");

    let config = MidasConfig::from_env()?;

    info!(
        "Model provider : {:?}",
        config.model_provider
    );

    info!(
        "Model URL : {}",
        config.model_url
    );

    info!(
        "Model : {}",
        config.model_name
    );

    info!(
        "Mémoire : {}",
        config.memory_path
    );

    let mut midas =
        MidasCore::new(config)?;

    midas.identity.set_operational();
    midas.state.set_running();

    info!(
        "Identité : {}",
        midas.identity.summary()
    );

    info!(
        "MIDAS opérationnel."
    );

    info!(
        "Capacités natives : {}",
        midas.capabilities.available().len()
    );

    info!(
        "Mémoire : {} entrée(s)",
        midas.memory.count()?
    );

    info!("Service permanent démarré.");
    info!("En attente d'événements...");

    run_service_loop(&mut midas).await?;

    midas.state.set_stopped();
    midas.identity.set_stopped();

    info!("MIDAS arrêté proprement.");

    Ok(())
}

async fn run_service_loop(
    midas: &mut MidasCore,
) -> Result<()> {
    let mut interval =
        tokio::time::interval(
            std::time::Duration::from_secs(5),
        );

    loop {
        tokio::select! {
            _ = interval.tick() => {
                heartbeat(midas)?;
            }

            result = signal::ctrl_c() => {
                match result {
                    Ok(()) => {
                        info!(
                            "Signal d'arrêt reçu."
                        );

                        break;
                    }

                    Err(error) => {
                        error!(
                            "Impossible d'écouter \
                             le signal d'arrêt : {}",
                            error
                        );

                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

fn heartbeat(
    midas: &mut MidasCore,
) -> Result<()> {
    let perceptions =
        vec![
            Perception::new(
                "system",
                "Heartbeat interne : MIDAS est en fonctionnement.",
            ),
        ];

    let result =
        midas.cycle(perceptions)?;

    info!(
        "Cycle {} | réflexion={:?} | décision={:?} | action_exécutée={} | observation={} | correction={}",
        midas.state.cycle,
        result.reflection.depth,
        result.decision.verification_level,
        result.action.executed,
        result.observation.result,
        result.correction.action,
    );

    Ok(())
}

fn init_logging() {
    let filter =
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| {
                EnvFilter::new(
                    "info,midas_core=debug",
                )
            });

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
}
