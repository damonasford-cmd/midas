use midas_core::{
    config::MidasConfig,
    core::MidasCore,
    perception::{
        Perception,
        PerceptionKind,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config =
        MidasConfig::from_env()?;

    let mut midas =
        MidasCore::new(config)?;

    println!("=================================");
    println!("MIDAS");
    println!(
        "Public identity: {}",
        midas.identity.public_name
    );
    println!(
        "Version: {}",
        midas.identity.version
    );
    println!("=================================");

    let perception = Perception::new(
        PerceptionKind::Text,
        "local",
        "Initialisation du système MIDAS.",
    );

    let result =
        midas.cycle(vec![perception])?;

    println!(
        "Cycle: {}",
        midas.state.cycle
    );

    println!(
        "Understanding: {}",
        result.understanding.summary
    );

    println!(
        "Reflection: {}",
        result.reflection.reasoning
    );

    println!(
        "Action: {}",
        result.action.description
    );

    println!(
        "Learning: {}",
        result.learning.lesson
    );

    let reasoning = midas
        .reason_with_model(
            "Explique brièvement les informations \
             actuellement disponibles et indique \
             quelles informations supplémentaires \
             seraient utiles.",
        )
        .await?;

    println!(
        "\nModel reasoning:\n{}",
        reasoning
    );

    println!(
        "\nMemory entries: {}",
        midas.memory.count()?
    );

    Ok(())
}
