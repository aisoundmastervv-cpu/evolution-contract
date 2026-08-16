//! Provider-neutral cloud execution substrate.
//!
//! This module is infrastructure-only. It invokes an already approved executor
//! through an opaque interface and records operational execution artifacts
//! without assigning semantic meaning to infrastructure events.

use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

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
pub struct ExecutionTrace {
    pub execution: ExecutionIdentity,
    pub attempt: u32,
    pub machine_state: ArtifactId,
    pub trace_ref: ArtifactId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PersistedExecution {
    pub context: ExecutionContext,
    pub attempts: Vec<ExecutionAttempt>,
    pub machine_state_ref: ArtifactId,
    pub trace_ref: ArtifactId,
    pub evidence_ref: ArtifactId,
    pub trace: ExecutionTrace,
    pub audit_head: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecoveryDecision {
    Resume(ExecutionContext),
    FailClosed(OperationalFailure),
}

pub trait ReferenceExecutorInvoker {
    type Error: Into<OperationalFailure>;

    fn invoke(&mut self, context: &ExecutionContext) -> Result<(), Self::Error>;
}

/// Provider-neutral contract. Implementations may differ operationally but
/// must expose the same executor invocation semantics.
pub trait CloudProvider {
    fn execute<I: ReferenceExecutorInvoker>(
        &mut self,
        invoker: &mut I,
        store: &mut dyn ExecutionStore,
        context: ExecutionContext,
        trace_ref: ArtifactId,
        evidence_ref: ArtifactId,
    ) -> ExecutionAttempt;
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

/// Durable reference provider used by conformance tests. The file is an
/// append-only journal with a chained audit digest. The chain is evidence of
/// record linkage; it does not acquire semantic authority.
pub struct FileExecutionStore {
    path: PathBuf,
    executions: BTreeMap<ExecutionIdentity, PersistedExecution>,
    audit_head: String,
}

impl FileExecutionStore {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let mut store = Self {
            path,
            executions: BTreeMap::new(),
            audit_head: String::new(),
        };
        if store.path.exists() {
            let content = fs::read_to_string(&store.path)?;
            let mut previous = String::new();
            for line in content.lines().filter(|line| !line.is_empty()) {
                let mut parts = line.splitn(3, '\t');
                let hash = parts.next().ok_or_else(|| invalid_data("audit hash"))?;
                let prior = parts.next().ok_or_else(|| invalid_data("audit prior"))?;
                let payload = parts.next().ok_or_else(|| invalid_data("audit payload"))?;
                if prior != previous || digest(&format!("{}\n{}", prior, payload)) != hash {
                    return Err(invalid_data("audit chain mismatch"));
                }
                let record = decode_record(payload)?;
                previous = hash.to_string();
                store.executions.insert(record.context.execution.clone(), record);
            }
            store.audit_head = previous;
        }
        Ok(store)
    }

    fn append(&mut self, execution: PersistedExecution) -> io::Result<()> {
        let payload = encode_record(&execution);
        let hash = digest(&format!("{}\n{}", self.audit_head, payload));
        let line = format!("{}\t{}\t{}\n", hash, self.audit_head, payload);
        let mut file = OpenOptions::new().create(true).append(true).open(&self.path)?;
        file.write_all(line.as_bytes())?;
        file.sync_all()?;
        self.audit_head = hash.clone();
        let mut stored = execution;
        stored.audit_head = hash;
        self.executions
            .insert(stored.context.execution.clone(), stored);
        Ok(())
    }
}

impl ExecutionStore for FileExecutionStore {
    fn persist(&mut self, mut execution: PersistedExecution) {
        execution.audit_head = self.audit_head.clone();
        self.append(execution)
            .expect("durable execution persistence failed");
    }

    fn load(&self, execution: &ExecutionIdentity) -> Option<&PersistedExecution> {
        self.executions.get(execution)
    }
}

pub struct ReferenceProviderA;
pub struct ReferenceProviderB;

impl CloudProvider for ReferenceProviderA {
    fn execute<I: ReferenceExecutorInvoker>(
        &mut self,
        invoker: &mut I,
        store: &mut dyn ExecutionStore,
        context: ExecutionContext,
        trace_ref: ArtifactId,
        evidence_ref: ArtifactId,
    ) -> ExecutionAttempt {
        execute_once(invoker, store, context, trace_ref, evidence_ref)
    }
}

impl CloudProvider for ReferenceProviderB {
    fn execute<I: ReferenceExecutorInvoker>(
        &mut self,
        invoker: &mut I,
        store: &mut dyn ExecutionStore,
        context: ExecutionContext,
        trace_ref: ArtifactId,
        evidence_ref: ArtifactId,
    ) -> ExecutionAttempt {
        execute_once(invoker, store, context, trace_ref, evidence_ref)
    }
}

pub fn execute_once<I: ReferenceExecutorInvoker, S: ExecutionStore + ?Sized>(
    invoker: &mut I,
    store: &mut S,
    context: ExecutionContext,
    trace_ref: ArtifactId,
    evidence_ref: ArtifactId,
) -> ExecutionAttempt {
    let attempt_number = store
        .load(&context.execution)
        .map(|record| record.attempts.len() as u32 + 1)
        .unwrap_or(1);

    let outcome = match invoker.invoke(&context) {
        Ok(()) => ExecutionOutcome::Completed,
        Err(error) => ExecutionOutcome::Aborted {
            reason: error.into(),
        },
    };

    let attempt = ExecutionAttempt {
        execution: context.execution.clone(),
        attempt: attempt_number,
        context: context.clone(),
        outcome,
    };

    let trace = ExecutionTrace {
        execution: context.execution.clone(),
        attempt: attempt_number,
        machine_state: context.machine_state.clone(),
        trace_ref: trace_ref.clone(),
    };

    let mut attempts = store
        .load(&context.execution)
        .map(|record| record.attempts.clone())
        .unwrap_or_default();
    attempts.push(attempt.clone());

    let audit_head = store
        .load(&context.execution)
        .map(|record| record.audit_head.clone())
        .unwrap_or_default();

    store.persist(PersistedExecution {
        context: context.clone(),
        attempts,
        machine_state_ref: context.machine_state,
        trace_ref,
        evidence_ref,
        trace,
        audit_head,
    });

    attempt
}

pub fn retry_context(previous: &ExecutionAttempt) -> ExecutionContext {
    previous.context.clone()
}

pub fn recover(persisted: Option<&PersistedExecution>) -> RecoveryDecision {
    let Some(record) = persisted else {
        return RecoveryDecision::FailClosed(OperationalFailure::StateUnavailable);
    };

    if record.machine_state_ref != record.context.machine_state
        || record.trace.execution != record.context.execution
        || record.trace.machine_state != record.machine_state_ref
        || record.trace.attempt == 0
        || record.trace.attempt != record.attempts.last().map(|a| a.attempt).unwrap_or(0)
        || record.attempts.iter().any(|attempt| {
            attempt.execution != record.context.execution || attempt.context != record.context
        })
    {
        return RecoveryDecision::FailClosed(OperationalFailure::StateInconsistent);
    }

    RecoveryDecision::Resume(record.context.clone())
}

fn invalid_data(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
}

fn unescape(value: &str) -> String {
    let mut out = String::new();
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('t') => out.push('\t'),
                Some('n') => out.push('\n'),
                Some('\\') => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn digest(input: &str) -> String {
    // Deterministic, provider-neutral audit linkage. The trusted file itself
    // remains the evidence anchor; this is linkage, not semantic authority.
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}

fn encode_record(record: &PersistedExecution) -> String {
    let artifacts = record
        .context
        .artifacts
        .iter()
        .map(|(k, v)| format!("{}={}", escape(k), escape(&v.0)))
        .collect::<Vec<_>>()
        .join(",");
    let inputs = record
        .context
        .inputs
        .iter()
        .map(|v| escape(&v.0))
        .collect::<Vec<_>>()
        .join(",");
    let outcomes = record
        .attempts
        .iter()
        .map(|attempt| {
            let outcome = match &attempt.outcome {
                ExecutionOutcome::Completed => "completed".to_string(),
                ExecutionOutcome::Aborted { reason } => format!("aborted:{reason:?}"),
            };
            format!("{}:{}", attempt.attempt, outcome)
        })
        .collect::<Vec<_>>()
        .join(",");

    [
        escape(&record.context.execution.0),
        artifacts,
        escape(&record.context.executor.version.0),
        escape(&record.context.scope.0),
        inputs,
        escape(&record.context.machine_state.0),
        escape(&record.context.resource_policy.0),
        escape(&record.context.execution_policy.0),
        outcomes,
        escape(&record.machine_state_ref.0),
        escape(&record.trace_ref.0),
        escape(&record.evidence_ref.0),
        escape(&record.trace.execution.0),
        record.trace.attempt.to_string(),
        escape(&record.trace.machine_state.0),
        escape(&record.trace.trace_ref.0),
    ]
    .join("\t")
}

fn decode_record(payload: &str) -> io::Result<PersistedExecution> {
    let fields = payload.split('\t').map(unescape).collect::<Vec<_>>();
    if fields.len() != 16 {
        return Err(invalid_data("execution record field count"));
    }
    let mut artifacts = BTreeMap::new();
    if !fields[1].is_empty() {
        for item in fields[1].split(',') {
            let (key, value) = item
                .split_once('=')
                .ok_or_else(|| invalid_data("artifact identity"))?;
            artifacts.insert(key.to_string(), ArtifactId(value.to_string()));
        }
    }
    let inputs = if fields[4].is_empty() {
        Vec::new()
    } else {
        fields[4]
            .split(',')
            .map(|value| ArtifactId(value.to_string()))
            .collect()
    };
    let mut attempts = Vec::new();
    if !fields[8].is_empty() {
        for item in fields[8].split(',') {
            let (attempt, outcome) = item
                .split_once(':')
                .ok_or_else(|| invalid_data("attempt record"))?;
            let attempt_number = attempt
                .parse::<u32>()
                .map_err(|_| invalid_data("attempt number"))?;
            let parsed_outcome = if outcome == "completed" {
                ExecutionOutcome::Completed
            } else if let Some(reason) = outcome.strip_prefix("aborted:") {
                let reason = match reason {
                    "WorkerTerminated" => OperationalFailure::WorkerTerminated,
                    "NetworkInterrupted" => OperationalFailure::NetworkInterrupted,
                    "StorageUnavailable" => OperationalFailure::StorageUnavailable,
                    "SchedulerFailure" => OperationalFailure::SchedulerFailure,
                    "AuthorizationUnavailable" => OperationalFailure::AuthorizationUnavailable,
                    "StateUnavailable" => OperationalFailure::StateUnavailable,
                    "StateInconsistent" => OperationalFailure::StateInconsistent,
                    "ResourceExhausted" => OperationalFailure::ResourceExhausted,
                    _ => return Err(invalid_data("operational failure")),
                };
                ExecutionOutcome::Aborted { reason }
            } else {
                return Err(invalid_data("execution outcome"));
            };
            attempts.push(ExecutionAttempt {
                execution: ExecutionIdentity(fields[0].clone()),
                attempt: attempt_number,
                context: ExecutionContext {
                    execution: ExecutionIdentity(fields[0].clone()),
                    artifacts: artifacts.clone(),
                    executor: ExecutorIdentity {
                        version: ArtifactId(fields[2].clone()),
                    },
                    scope: ArtifactId(fields[3].clone()),
                    inputs: inputs.clone(),
                    machine_state: ArtifactId(fields[5].clone()),
                    resource_policy: ArtifactId(fields[6].clone()),
                    execution_policy: ArtifactId(fields[7].clone()),
                },
                outcome: parsed_outcome,
            });
        }
    }
    let trace_attempt = fields[13]
        .parse::<u32>()
        .map_err(|_| invalid_data("trace attempt"))?;
    Ok(PersistedExecution {
        context: ExecutionContext {
            execution: ExecutionIdentity(fields[0].clone()),
            artifacts,
            executor: ExecutorIdentity {
                version: ArtifactId(fields[2].clone()),
            },
            scope: ArtifactId(fields[3].clone()),
            inputs,
            machine_state: ArtifactId(fields[5].clone()),
            resource_policy: ArtifactId(fields[6].clone()),
            execution_policy: ArtifactId(fields[7].clone()),
        },
        attempts,
        machine_state_ref: ArtifactId(fields[9].clone()),
        trace_ref: ArtifactId(fields[10].clone()),
        evidence_ref: ArtifactId(fields[11].clone()),
        trace: ExecutionTrace {
            execution: ExecutionIdentity(fields[12].clone()),
            attempt: trace_attempt,
            machine_state: ArtifactId(fields[14].clone()),
            trace_ref: ArtifactId(fields[15].clone()),
        },
        audit_head: String::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestInvoker {
        calls: usize,
        failure: Option<OperationalFailure>,
    }

    impl ReferenceExecutorInvoker for TestInvoker {
        type Error = OperationalFailure;

        fn invoke(&mut self, _context: &ExecutionContext) -> Result<(), Self::Error> {
            self.calls += 1;
            match self.failure.clone() {
                Some(error) => Err(error),
                None => Ok(()),
            }
        }
    }

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
    fn executor_invocation_is_recorded_as_an_execution_attempt() {
        let ctx = context();
        let mut invoker = TestInvoker {
            calls: 0,
            failure: None,
        };
        let mut store = InMemoryExecutionStore::new();
        let attempt = execute_once(
            &mut invoker,
            &mut store,
            ctx.clone(),
            ArtifactId("trace@1".into()),
            ArtifactId("evidence@1".into()),
        );
        assert_eq!(invoker.calls, 1);
        assert_eq!(attempt.attempt, 1);
        assert_eq!(attempt.execution, ctx.execution);
        assert_eq!(attempt.outcome, ExecutionOutcome::Completed);
        assert_eq!(store.load(&attempt.execution).unwrap().attempts.len(), 1);
    }

    #[test]
    fn operational_failure_remains_non_semantic() {
        let ctx = context();
        let mut invoker = TestInvoker {
            calls: 0,
            failure: Some(OperationalFailure::NetworkInterrupted),
        };
        let mut store = InMemoryExecutionStore::new();
        let attempt = execute_once(
            &mut invoker,
            &mut store,
            ctx,
            ArtifactId("trace@1".into()),
            ArtifactId("evidence@1".into()),
        );
        assert_eq!(
            attempt.outcome,
            ExecutionOutcome::Aborted {
                reason: OperationalFailure::NetworkInterrupted
            }
        );
    }

    #[test]
    fn retry_reuses_context_and_creates_distinct_attempt() {
        let ctx = context();
        let mut invoker = TestInvoker {
            calls: 0,
            failure: Some(OperationalFailure::WorkerTerminated),
        };
        let mut store = InMemoryExecutionStore::new();
        let first = execute_once(
            &mut invoker,
            &mut store,
            ctx.clone(),
            ArtifactId("trace@1".into()),
            ArtifactId("evidence@1".into()),
        );
        let retry = execute_once(
            &mut invoker,
            &mut store,
            retry_context(&first),
            ArtifactId("trace@2".into()),
            ArtifactId("evidence@2".into()),
        );
        assert_eq!(first.context, retry.context);
        assert_eq!(first.attempt, 1);
        assert_eq!(retry.attempt, 2);
        assert_eq!(store.load(&ctx.execution).unwrap().attempts.len(), 2);
    }

    #[test]
    fn recovery_missing_state_fails_closed() {
        assert_eq!(
            recover(None),
            RecoveryDecision::FailClosed(OperationalFailure::StateUnavailable)
        );
    }

    #[test]
    fn recovery_inconsistent_state_fails_closed() {
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
            trace: ExecutionTrace {
                execution: ctx.execution.clone(),
                attempt: 1,
                machine_state: ctx.machine_state.clone(),
                trace_ref: ArtifactId("trace@1".into()),
            },
            audit_head: String::new(),
        };
        assert_eq!(
            recover(Some(&record)),
            RecoveryDecision::FailClosed(OperationalFailure::StateInconsistent)
        );
    }

    #[test]
    fn state_and_trace_correspondence_is_required() {
        let ctx = context();
        let record = PersistedExecution {
            context: ctx.clone(),
            attempts: vec![ExecutionAttempt {
                execution: ctx.execution.clone(),
                attempt: 1,
                context: ctx.clone(),
                outcome: ExecutionOutcome::Completed,
            }],
            machine_state_ref: ctx.machine_state.clone(),
            trace_ref: ArtifactId("trace@1".into()),
            evidence_ref: ArtifactId("evidence@1".into()),
            trace: ExecutionTrace {
                execution: ctx.execution.clone(),
                attempt: 1,
                machine_state: ArtifactId("state@different".into()),
                trace_ref: ArtifactId("trace@1".into()),
            },
            audit_head: String::new(),
        };
        assert_eq!(
            recover(Some(&record)),
            RecoveryDecision::FailClosed(OperationalFailure::StateInconsistent)
        );
    }

    #[test]
    fn durable_store_round_trips_across_reopen() {
        let path = std::env::temp_dir().join(format!(
            "evolution-cloud-{}.journal",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let ctx = context();
        {
            let mut store = FileExecutionStore::open(&path).unwrap();
            let mut invoker = TestInvoker {
                calls: 0,
                failure: None,
            };
            execute_once(
                &mut invoker,
                &mut store,
                ctx.clone(),
                ArtifactId("trace@1".into()),
                ArtifactId("evidence@1".into()),
            );
        }
        let reopened = FileExecutionStore::open(&path).unwrap();
        let record = reopened.load(&ctx.execution).unwrap();
        assert_eq!(record.context, ctx);
        assert_eq!(recover(Some(record)), RecoveryDecision::Resume(ctx));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn tampered_audit_journal_is_rejected() {
        let path = std::env::temp_dir().join(format!(
            "evolution-cloud-tamper-{}.journal",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let ctx = context();
        {
            let mut store = FileExecutionStore::open(&path).unwrap();
            let mut invoker = TestInvoker {
                calls: 0,
                failure: None,
            };
            execute_once(
                &mut invoker,
                &mut store,
                ctx,
                ArtifactId("trace@1".into()),
                ArtifactId("evidence@1".into()),
            );
        }
        let mut bytes = fs::read(&path).unwrap();
        if let Some(byte) = bytes.iter_mut().find(|byte| **byte == b'e') {
            *byte = b'x';
        }
        fs::write(&path, bytes).unwrap();
        assert!(FileExecutionStore::open(&path).is_err());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn provider_substitution_preserves_execution_semantics() {
        let ctx = context();
        let mut a_invoker = TestInvoker {
            calls: 0,
            failure: None,
        };
        let mut b_invoker = TestInvoker {
            calls: 0,
            failure: None,
        };
        let mut a_store = InMemoryExecutionStore::new();
        let mut b_store = InMemoryExecutionStore::new();
        let mut provider_a = ReferenceProviderA;
        let mut provider_b = ReferenceProviderB;
        let a = provider_a.execute(
            &mut a_invoker,
            &mut a_store,
            ctx.clone(),
            ArtifactId("trace@a".into()),
            ArtifactId("evidence@a".into()),
        );
        let b = provider_b.execute(
            &mut b_invoker,
            &mut b_store,
            ctx,
            ArtifactId("trace@b".into()),
            ArtifactId("evidence@b".into()),
        );
        assert_eq!(a.context, b.context);
        assert_eq!(a.outcome, b.outcome);
        assert_eq!(a.attempt, b.attempt);
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
            trace: ExecutionTrace {
                execution: ctx.execution.clone(),
                attempt: 1,
                machine_state: ctx.machine_state.clone(),
                trace_ref: ArtifactId("trace@1".into()),
            },
            audit_head: String::new(),
        };
        let mut store = InMemoryExecutionStore::new();
        store.persist(record.clone());
        assert_eq!(store.load(&ctx.execution), Some(&record));
    }
}
