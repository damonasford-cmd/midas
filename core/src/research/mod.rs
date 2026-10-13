
pub mod discovery;
pub mod domain;
pub mod evidence;
pub mod experiments;
pub mod hypotheses;
pub mod knowledge_bridge;
pub mod methods;
pub mod objectives;
pub mod orchestration;
pub mod repository;

pub use discovery::{
    Discovery,
    DiscoveryKind,
    ResearchReport,
};
pub use domain::{
    ResearchMethodKind,
    ResearchObjective,
    ResearchRun,
    ResearchStatus,
    ResearchTask,
    TaskStatus,
};
pub use evidence::{
    Evidence,
    EvidencePolarity,
    VerificationLevel,
};
pub use experiments::{
    ExperimentKind,
    ExperimentPlan,
    ExperimentStatus,
};
pub use hypotheses::{
    Hypothesis,
    HypothesisStatus,
};
pub use knowledge_bridge::{
    KnowledgeCandidate,
    KnowledgePublisher,
};
pub use methods::{
    MethodDescriptor,
    MethodResult,
    ResearchMethod,
    ResearchMethodRegistry,
};
pub use orchestration::ResearchEngine;
pub use repository::ResearchRepository;
