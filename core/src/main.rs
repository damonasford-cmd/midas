use midas_core::{
    core::MidasCore,
    perception::{Perception, PerceptionKind},
};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let mut midas = MidasCore::new("data/memory/midas.memory")?;

    println!("=================================");
    println!("MIDAS");
    println!("Public identity: {}", midas.identity.public_name);
    println!("Version: {}", midas.identity.version);
    println!("=================================");

    let perception = Perception::new(
        PerceptionKind::Text,
        "local",
        "Initialisation du système MIDAS.",
    );

    let result = midas.cycle(vec![perception])?;

    println!("Cycle: {}", midas.state.cycle);
    println!("Understanding: {}", result.understanding.summary);
    println!("Reflection: {}", result.reflection.reasoning);
    println!("Action: {}", result.action.description);
    println!("Learning: {}", result.learning.lesson);
    println!(
        "Memory entries: {}",
        midas.memory.count()?
    );

    Ok(())
}
