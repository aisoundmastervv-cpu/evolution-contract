//! Cloud boundary adapter for the approved Reference Executor.
//!
//! This adapter binds an execution request to an explicitly authorized
//! executor identity before invocation. It does not define executor semantics.

#[path = "cloud_execution.rs"]
mod substrate;

pub use substrate::{
    ArtifactId, ExecutionAttempt, ExecutionContext, ExecutionIdentity, ExecutionOutcome,
    ExecutionStore, InMemoryExecutionStore, OperationalFailure, ReferenceExecutorInvoker,
};

pub fn execute_approved_once<I: ReferenceExecutorInvoker, S: ExecutionStore>(
    invoker: &mut I,
    store: &mut S,
    context: ExecutionContext,
    approved_executor_version: &ArtifactId,
    trace_ref: ArtifactId,
    evidence_ref: ArtifactId,
) -> ExecutionAttempt {
    if &context.executor.version != approved_executor_version {
        let attempt_number = store
            .load(&context.execution)
            .map(|record| record.attempts.len() as u32 + 1)
            .unwrap_or(1);
        let attempt = ExecutionAttempt {
            execution: context.execution.clone(),
            attempt: attempt_number,
            context: context.clone(),
            outcome: ExecutionOutcome::Aborted {
                reason: OperationalFailure::AuthorizationUnavailable,
            },
        };
        store.persist(substrate::PersistedExecution {
            context: context.clone(),
            attempts: vec![attempt.clone()],
            machine_state_ref: context.machine_state,
            trace_ref,
            evidence_ref,
        });
        return attempt;
    }

    substrate::execute_once(invoker, store, context, trace_ref, evidence_ref)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestInvoker {
        calls: usize,
    }

    impl ReferenceExecutorInvoker for TestInvoker {
        type Error = OperationalFailure;

        fn invoke(&mut self, _context: &ExecutionContext) -> Result<(), Self::Error> {
            self.calls += 1;
            Ok(())
        }
    }

    fn context(version: &str) -> ExecutionContext {
        ExecutionContext {
            execution: ExecutionIdentity("exec-1".into()),
            artifacts: [
                ("contract".into(), ArtifactId("contract@frozen".into())),
                (
                    "state-model".into(),
                    ArtifactId("state-model@approved".into()),
                ),
                (
                    "executor-spec".into(),
                    ArtifactId("executor-spec@approved".into()),
                ),
            ]
            .into_iter()
            .collect(),
            executor: substrate::ExecutorIdentity {
                version: ArtifactId(version.into()),
            },
            scope: ArtifactId("scope@g11".into()),
            inputs: vec![ArtifactId("input@1".into())],
            machine_state: ArtifactId("state@observation-required".into()),
            resource_policy: ArtifactId("resources@default".into()),
            execution_policy: ArtifactId("policy@retry-safe".into()),
        }
    }

    #[test]
    fn approved_executor_identity_is_required_before_invocation() {
        let mut invoker = TestInvoker { calls: 0 };
        let mut store = InMemoryExecutionStore::new();
        let approved = ArtifactId("executor@approved".into());

        let attempt = execute_approved_once(
            &mut invoker,
            &mut store,
            context("executor@different"),
            &approved,
            ArtifactId("trace@1".into()),
            ArtifactId("evidence@1".into()),
        );

        assert_eq!(invoker.calls, 0);
        assert_eq!(
            attempt.outcome,
            ExecutionOutcome::Aborted {
                reason: OperationalFailure::AuthorizationUnavailable
            }
        );
    }

    #[test]
    fn approved_executor_identity_allows_invocation() {
        let mut invoker = TestInvoker { calls: 0 };
        let mut store = InMemoryExecutionStore::new();
        let approved = ArtifactId("executor@approved".into());

        let attempt = execute_approved_once(
            &mut invoker,
            &mut store,
            context("executor@approved"),
            &approved,
            ArtifactId("trace@1".into()),
            ArtifactId("evidence@1".into()),
        );

        assert_eq!(invoker.calls, 1);
        assert_eq!(attempt.outcome, ExecutionOutcome::Completed);
    }
}
