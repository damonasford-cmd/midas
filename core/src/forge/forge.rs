use crate::forge::{
    BenchmarkEngine,
    CodeGenerator,
    Compiler,
    Debugger,
    DeploymentEngine,
    EvolutionEngine,
    TestRunner,
};

#[derive(Debug, Clone)]
pub struct Forge {
    pub code_generator: CodeGenerator,
    pub compiler: Compiler,
    pub tester: TestRunner,
    pub benchmarker: BenchmarkEngine,
    pub debugger: Debugger,
    pub deployment: DeploymentEngine,
    pub evolution: EvolutionEngine,
}

impl Default for Forge {
    fn default() -> Self {
        Self {
            code_generator: CodeGenerator::new(),
            compiler: Compiler::new(),
            tester: TestRunner::new(),
            benchmarker: BenchmarkEngine::new(),
            debugger: Debugger::new(),
            deployment: DeploymentEngine::new(),
            evolution: EvolutionEngine::new(),
        }
    }
}

impl Forge {
    pub fn new() -> Self {
        Self::default()
    }
}
