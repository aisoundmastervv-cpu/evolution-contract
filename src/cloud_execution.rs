//! Provider-neutral cloud execution substrate.
//!
//! This module is infrastructure-only. It invokes an already approved executor
//! through an opaque interface and records operational execution artifacts
//! without assigning semantic meaning to infrastructure events.

use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArtifactId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExecutionIdentity(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutorIdentity {
    pub version: ArtifactId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionContext {
    pub execution: ExecutionIdentity,
    pub artifacts: BTreeMap<String, ArtifactId>,
    pub executor: ExecutorIdentity,
    pub scope: ArtifactId,
    pub inputs: Vec<ArtifactId>,
    pub machine_state: ArtifactId,
    pub resource_policy: ArtifactId,
    pub execution_policy: ArtifactId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecutionOutcome {
    Completed,
    Aborted { reason: OperationalFailure },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OperationalFailure {
    WorkerTerminated,
    NetworkInterrupted,
    StorageUnavailable,
    SchedulerFailure,
    AuthorizationUnavailable,
    StateUnavailable,
    StateInconsistent,
    ResourceExhausted,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionAttempt {
    pub execution: ExecutionIdentity,
    pub attempt: u32,
    pub context: ExecutionContext,
    pub outcome: ExecutionOutcome,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PersistedExecution {
    pub context: ExecutionContext,
    pub attempts: Vec<ExecutionAttempt>,
    pub machine_state_ref: ArtifactId,
    pub trace_ref: ArtifactId,
    pub evidence_ref: ArtifactId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecoveryDecision {
    Resume(ExecutionContext),
    FailClosed(OperationalFailure),
}

pub trait ReferenceExecutorInvoker {
    type Error;

    fn invoke(&mut self, context: &ExecutionContext) -> Result<(), Self::Error>;
}

pub trait ExecutionStore {
    fn persist(&mut self, execution: PersistedExecution);
    fn load(&self, execution: &ExecutionIdentity) -> Option<&PersistedExecution>;
}

pub struct InMemoryExecutionStore {
    executions: BTreeMap<ExecutionIdentity, PersistedExecution>,
}

impl InMemoryExecutionStore {
    pub fn new() -> Self {
        Self {
            executions: BTreeMap::new(),
        }
    }
}

impl Default for InMemoryExecutionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionStore for InMemoryExecutionStore {
    fn persist(&mut self, execution: PersistedExecution) {
        self.executions
            .insert(execution.context.execution.clone(), execution);
    }

    fn load(&self, execution: &ExecutionIdentity) -> Option<&PersistedExecution> {
        self.executions.get(execution)
    }
}

pub fn retry_context(previous: &ExecutionAttempt) -> ExecutionContext {
    previous.context.clone()
}

pub fn recover(persisted: Option<&PersistedExecution>) -> RecoveryDecision {
    let Some(record) = persisted else {
        return RecoveryDecision::FailClosed(OperationalFailure::StateUnavailable);
    };

    if record
        .attempts
        .iter()
        .any(|attempt| attempt.context != record.context)
    {
        return RecoveryDecision::FailClosed(OperationalFailure::StateInconsistent);
    }

    RecoveryDecision::Resume(record.context.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context() -> ExecutionContext {
        let mut artifacts = BTreeMap::new();
        artifacts.insert("contract".into(), ArtifactId("contract@frozen".into()));
        artifacts.insert(
            "state-model".into(),
            ArtifactId("state-model@approved".into()),
        );
        artifacts.insert(
            "executor-spec".into(),
            ArtifactId("executor-spec@approved".into()),
        );

        ExecutionContext {
            execution: ExecutionIdentity("exec-1".into()),
            artifacts,
            executor: ExecutorIdentity {
                version: ArtifactId("executor@approved".into()),
            },
            scope: ArtifactId("scope@g11".into()),
            inputs: vec![ArtifactId("input@1".into())],
            machine_state: ArtifactId("state@observation-required".into()),
            resource_policy: ArtifactId("resources@default".into()),
            execution_policy: ArtifactId("policy@retry-safe".into()),
        }
    }

    #[test]
    fn retry_reuses_authorized_context() {
        let ctx = context();
        let attempt = ExecutionAttempt {
            execution: ctx.execution.clone(),
            attempt: 1,
            context: ctx.clone(),
            outcome: ExecutionOutcome::Aborted {
                reason: OperationalFailure::WorkerTerminated,
            },
        };

        assert_eq!(retry_context(&attempt), ctx);
    }

    #[test]
    fn operational_failure_remains_non_semantic() {
        let ctx = context();
        let attempt = ExecutionAttempt {
            execution: ctx.execution.clone(),
            attempt: 1,
            context: ctx,
            outcome: ExecutionOutcome::Aborted {
                reason: OperationalFailure::NetworkInterrupted,
            },
        };

        assert!(matches!(
            attempt.outcome,
            ExecutionOutcome::Aborted {
                reason: OperationalFailure::NetworkInterrupted
            }
        ));
    }

    #[test]
    fn recovery_missing_state_fails_closed() {
        assert_eq!(
            recover(None),
            RecoveryDecision::FailClosed(OperationalFailure::StateUnavailable)
        );
    }

    #[test]
    fn recovery_inconsistent_attempt_fails_closed() {
        let ctx = context();
        let mut attempt_context = ctx.clone();
        attempt_context.machine_state = ArtifactId("state@different".into());
        let attempt = ExecutionAttempt {
            execution: ctx.execution.clone(),
            attempt: 1,
            context: attempt_context,
            outcome: ExecutionOutcome::Aborted {
                reason: OperationalFailure::WorkerTerminated,
            },
        };
        let record = PersistedExecution {
            context: ctx.clone(),
            attempts: vec![attempt],
            machine_state_ref: ctx.machine_state.clone(),
            trace_ref: ArtifactId("trace@1".into()),
            evidence_ref: ArtifactId("evidence@1".into()),
        };

        assert_eq!(
            recover(Some(&record)),
            RecoveryDecision::FailClosed(OperationalFailure::StateInconsistent)
        );
    }

    #[test]
    fn store_round_trips_execution_record() {
        let ctx = context();
        let record = PersistedExecution {
            context: ctx.clone(),
            attempts: Vec::new(),
            machine_state_ref: ctx.machine_state.clone(),
            trace_ref: ArtifactId("trace@1".into()),
            evidence_ref: ArtifactId("evidence@1".into()),
        };
        let mut store = InMemoryExecutionStore::new();
        store.persist(record.clone());
        assert_eq!(store.load(&ctx.execution), Some(&record));
    }
}
