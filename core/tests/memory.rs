use midas_core::memory::Memory;

#[test]
fn memory_stores_and_retrieves_entries() {
    let mut memory = Memory::new();

    let id = memory.store(
        1,
        "test",
        "MIDAS apprend quelque chose",
    );

    assert_eq!(memory.len(), 1);

    let entry = memory
        .get(id)
        .expect("memory entry should exist");

    assert_eq!(entry.id, id);
    assert_eq!(entry.cycle, 1);
    assert_eq!(entry.category, "test");
    assert_eq!(
        entry.content,
        "MIDAS apprend quelque chose"
    );
}

#[test]
fn memory_can_search() {
    let mut memory = Memory::new();

    memory.store(
        1,
        "learning",
        "MIDAS apprend Rust",
    );

    memory.store(
        2,
        "decision",
        "MIDAS prépare une décision",
    );

    let results = memory.search("Rust");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].content, "MIDAS apprend Rust");
}
