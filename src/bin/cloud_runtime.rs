//! Minimal operational entrypoint for the provider-neutral cloud execution runtime.
//!
//! This binary is an operational adapter around the approved validation executor.
//! It does not define semantic state-machine rules; it only decodes a bounded
//! runtime request, invokes the approved executor through the cloud boundary,
//! and persists the resulting operational attempt.

#[path = "../cloud_executor_adapter.rs"]
mod cloud_executor_adapter;
#[path = "../validation_executor.rs"]
mod validation_executor;

use cloud_executor_adapter::{
    execute_approved_once, ArtifactId, ExecutionContext, ExecutionIdentity, ExecutionOutcome,
    ExecutorIdentity, FileExecutionStore, OperationalFailure, ReferenceExecutorInvoker,
};
use validation_executor::{ExecutionOutcome as ExecutorOutcome, Executor, MachineState, Transition};

struct ValidationExecutorInvoker {
    executor: Executor,
    transition: Transition,
}

impl ReferenceExecutorInvoker for ValidationExecutorInvoker {
    type Error = OperationalFailure;

    fn invoke(&mut self, _context: &ExecutionContext) -> Result<(), Self::Error> {
        match self.executor.apply(self.transition) {
            ExecutorOutcome::Transitioned(_) => Ok(()),
            ExecutorOutcome::Unauthorized { .. } => {
                Err(OperationalFailure::AuthorizationUnavailable)
            }
            ExecutorOutcome::ExecutionAborted => Err(OperationalFailure::SchedulerFailure),
        }
    }
}

fn main() {
    let execution_id = argument_value("--execution-id")
        .unwrap_or_else(|| "runtime-exec-1".to_string());
    let transition_name = argument_value("--transition")
        .unwrap_or_else(|| "observation-unavailable".to_string());
    let journal_path = argument_value("--journal")
        .unwrap_or_else(|| "cloud-runtime.journal".to_string());

    let transition = match transition_from_name(&transition_name) {
        Some(transition) => transition,
        None => {
            eprintln!(
                "unsupported --transition '{}'; supported values: observation-execute, observation-unavailable",
                transition_name
            );
            std::process::exit(2);
        }
    };

    let context = runtime_context(ExecutionIdentity(execution_id));
    let mut invoker = ValidationExecutorInvoker {
        executor: Executor::new(MachineState::ObservationRequired),
        transition,
    };
    let mut store = match FileExecutionStore::open(&journal_path) {
        Ok(store) => store,
        Err(error) => {
            eprintln!("failed to open execution journal: {error}");
            std::process::exit(1);
        }
    };

    let attempt = execute_approved_once(
        &mut invoker,
        &mut store,
        context,
        &ArtifactId("executor@approved".into()),
        ArtifactId("trace@runtime".into()),
        ArtifactId("evidence@runtime".into()),
    );

    match attempt.outcome {
        ExecutionOutcome::Completed => {
            println!(
                "execution={} attempt={} outcome=completed",
                attempt.execution.0, attempt.attempt
            );
        }
        ExecutionOutcome::Aborted { reason } => {
            eprintln!(
                "execution={} attempt={} outcome=aborted reason={reason:?}",
                attempt.execution.0, attempt.attempt
            );
            std::process::exit(1);
        }
    }
}

fn argument_value(name: &str) -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == name {
            return args.next();
        }
    }
    None
}

fn transition_from_name(name: &str) -> Option<Transition> {
    match name {
        "observation-execute" => Some(Transition::ObservationExecute),
        "observation-unavailable" => Some(Transition::ObservationUnavailable),
        _ => None,
    }
}

fn runtime_context(execution: ExecutionIdentity) -> ExecutionContext {
    ExecutionContext {
        execution,
        artifacts: [
            (
                "contract".into(),
                ArtifactId("contract@frozen".into()),
            ),
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
        executor: ExecutorIdentity {
            version: ArtifactId("executor@approved".into()),
        },
        scope: ArtifactId("scope@production-candidate".into()),
        inputs: vec![ArtifactId("runtime-input@1".into())],
        machine_state: ArtifactId("state@observation-required".into()),
        resource_policy: ArtifactId("resources@default".into()),
        execution_policy: ArtifactId("policy@retry-safe".into()),
    }
}
