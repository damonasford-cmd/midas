use midas_core::{
    loop_engine::MidasCore,
    state::CoreStatus,
};

#[test]
fn core_runs_a_complete_cycle() {
    let mut core = MidasCore::default();

    let result = core.run_cycle(
        "tester le cycle",
        "entrée de test",
    );

    assert!(result.contains("Cycle 1 terminé"));
    assert_eq!(core.state.cycle, 1);
    assert_eq!(core.state.status, CoreStatus::Ready);
    assert!(core.state.last_decision.is_some());
    assert!(core.state.last_result.is_some());
}

#[test]
fn core_has_native_capabilities() {
    let core = MidasCore::default();

    assert!(core.capabilities.has("perception"));
    assert!(core.capabilities.has("reasoning"));
    assert!(core.capabilities.has("learning"));
    assert!(core.capabilities.has("correction"));
}
