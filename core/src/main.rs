use midas_core::MidasCore;

fn main() {
    let mut midas = MidasCore::default();

    println!(
        "MIDAS — {} — prêt.",
        midas.identity.public_name
    );

    let result = midas.run_cycle(
        "Construire le noyau opérationnel de MIDAS",
        "Le système démarre son premier cycle interne.",
    );

    println!("{result}");
}
