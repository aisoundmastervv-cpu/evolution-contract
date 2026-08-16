//! Reference executor for the approved Validation Machine State Model v0.1.
//!
//! This module contains execution mechanics only. It does not define Contract
//! semantics, observation semantics, evidence admissibility, or oracle criteria.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MachineState {
    FrozenInput,
    PlanAuthorized,
    ObservationRequired,
    ObservationExecuted,
    EvidenceCollected,
    OracleEvaluated,
    Verdict,
    ObservationUnavailable,
    ObservationGap,
    Underdetermined,
    ForbiddenTransition,
    NotAuthorized,
    Stop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    Fail,
    Untested,
    ObservationGap,
    Underdetermined,
    NotAuthorized,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transition {
    PlanAuthorize,
    RequireObservation,
    ObservationExecute,
    ObservationUnavailable,
    CollectEvidence,
    EvaluateOracle,
    Verdict(Verdict),
    MarkUnderdetermined,
    ClassifyObservationGap,
    Stop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransitionTrace {
    states: [MachineState; 3],
    len: usize,
}

impl TransitionTrace {
    const fn unauthorized_guard() -> Self {
        Self {
            states: [
                MachineState::ForbiddenTransition,
                MachineState::NotAuthorized,
                MachineState::Stop,
            ],
            len: 3,
        }
    }

    pub const fn len(self) -> usize {
        self.len
    }

    pub const fn states(self) -> [MachineState; 3] {
        self.states
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionOutcome {
    Transitioned(MachineState),
    Unauthorized { trace: TransitionTrace },
    ExecutionAborted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Executor {
    state: MachineState,
}

impl Executor {
    pub const fn new(state: MachineState) -> Self {
        Self { state }
    }

    pub const fn state(self) -> MachineState {
        self.state
    }

    /// Attempts one authorized machine transition.
    ///
    /// Authorization is checked before the requested transition is applied.
    /// An unauthorized request follows the State Model's global guard and
    /// cannot execute the requested action.
    pub fn apply(&mut self, transition: Transition) -> ExecutionOutcome {
        match authorized_next_state(self.state, transition) {
            Some(next) => {
                self.state = next;
                ExecutionOutcome::Transitioned(next)
            }
            None => {
                self.state = MachineState::ForbiddenTransition;
                self.state = MachineState::NotAuthorized;
                self.state = MachineState::Stop;
                ExecutionOutcome::Unauthorized {
                    trace: TransitionTrace::unauthorized_guard(),
                }
            }
        }
    }

    /// Records a technical execution abort without creating a semantic state
    /// transition or verdict. The machine state is preserved.
    pub const fn abort_execution(&self) -> ExecutionOutcome {
        ExecutionOutcome::ExecutionAborted
    }
}

fn authorized_next_state(
    state: MachineState,
    transition: Transition,
) -> Option<MachineState> {
    use MachineState::*;

    match (state, transition) {
        (FrozenInput, Transition::PlanAuthorize) => Some(PlanAuthorized),
        (PlanAuthorized, Transition::RequireObservation) => Some(ObservationRequired),
        (PlanAuthorized, Transition::Verdict(Verdict::Untested)) => Some(MachineState::Verdict),
        (ObservationRequired, Transition::ObservationExecute) => Some(ObservationExecuted),
        (ObservationRequired, Transition::ObservationUnavailable) => Some(ObservationUnavailable),
        (ObservationExecuted, Transition::CollectEvidence) => Some(EvidenceCollected),
        (EvidenceCollected, Transition::EvaluateOracle) => Some(OracleEvaluated),
        (EvidenceCollected, Transition::MarkUnderdetermined) => Some(Underdetermined),
        (OracleEvaluated, Transition::Verdict(_)) => Some(Verdict),
        (OracleEvaluated, Transition::MarkUnderdetermined) => Some(Underdetermined),
        (ObservationUnavailable, Transition::ClassifyObservationGap) => Some(ObservationGap),
        (ObservationGap, Transition::Stop) => Some(Stop),
        (Underdetermined, Transition::Stop) => Some(Stop),
        (NotAuthorized, Transition::Stop) => Some(Stop),
        (Verdict, Transition::Stop) => Some(Stop),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorized_path_reaches_verdict_and_stop() {
        let mut executor = Executor::new(MachineState::FrozenInput);

        assert_eq!(executor.apply(Transition::PlanAuthorize), ExecutionOutcome::Transitioned(MachineState::PlanAuthorized));
        assert_eq!(executor.apply(Transition::RequireObservation), ExecutionOutcome::Transitioned(MachineState::ObservationRequired));
        assert_eq!(executor.apply(Transition::ObservationExecute), ExecutionOutcome::Transitioned(MachineState::ObservationExecuted));
        assert_eq!(executor.apply(Transition::CollectEvidence), ExecutionOutcome::Transitioned(MachineState::EvidenceCollected));
        assert_eq!(executor.apply(Transition::EvaluateOracle), ExecutionOutcome::Transitioned(MachineState::OracleEvaluated));
        assert_eq!(executor.apply(Transition::Verdict(Verdict::Pass)), ExecutionOutcome::Transitioned(MachineState::Verdict));
        assert_eq!(executor.apply(Transition::Stop), ExecutionOutcome::Transitioned(MachineState::Stop));
    }

    #[test]
    fn unauthorized_transition_exposes_full_guard_trace() {
        let mut executor = Executor::new(MachineState::ObservationRequired);

        assert_eq!(
            executor.apply(Transition::Verdict(Verdict::Pass)),
            ExecutionOutcome::Unauthorized {
                trace: TransitionTrace {
                    states: [
                        MachineState::ForbiddenTransition,
                        MachineState::NotAuthorized,
                        MachineState::Stop,
                    ],
                    len: 3,
                },
            }
        );
        assert_eq!(executor.state(), MachineState::Stop);
    }

    #[test]
    fn observation_gap_is_terminal() {
        let mut executor = Executor::new(MachineState::ObservationRequired);

        assert_eq!(executor.apply(Transition::ObservationUnavailable), ExecutionOutcome::Transitioned(MachineState::ObservationUnavailable));
        assert_eq!(executor.apply(Transition::ClassifyObservationGap), ExecutionOutcome::Transitioned(MachineState::ObservationGap));
        assert_eq!(executor.apply(Transition::Stop), ExecutionOutcome::Transitioned(MachineState::Stop));
    }

    #[test]
    fn underdetermined_path_is_terminal() {
        let mut executor = Executor::new(MachineState::EvidenceCollected);

        assert_eq!(executor.apply(Transition::MarkUnderdetermined), ExecutionOutcome::Transitioned(MachineState::Underdetermined));
        assert_eq!(executor.apply(Transition::Stop), ExecutionOutcome::Transitioned(MachineState::Stop));
    }

    #[test]
    fn untested_is_a_verdict_classification_not_a_state_transition_from_execution_failure() {
        let mut executor = Executor::new(MachineState::PlanAuthorized);

        assert_eq!(executor.apply(Transition::Verdict(Verdict::Untested)), ExecutionOutcome::Transitioned(MachineState::Verdict));
        assert_eq!(executor.apply(Transition::Stop), ExecutionOutcome::Transitioned(MachineState::Stop));
    }

    #[test]
    fn execution_abort_preserves_machine_state_and_has_no_semantic_verdict() {
        let executor = Executor::new(MachineState::ObservationRequired);

        assert_eq!(executor.abort_execution(), ExecutionOutcome::ExecutionAborted);
        assert_eq!(executor.state(), MachineState::ObservationRequired);
    }

    #[test]
    fn terminal_states_do_not_accept_implicit_retry() {
        let mut executor = Executor::new(MachineState::Stop);

        assert!(matches!(
            executor.apply(Transition::RequireObservation),
            ExecutionOutcome::Unauthorized { .. }
        ));
        assert_eq!(executor.state(), MachineState::Stop);
    }
}
