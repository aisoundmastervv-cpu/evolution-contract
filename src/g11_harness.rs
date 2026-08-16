//! G11 harness: operationalizes frozen O1 predicates only.
//! This module is test-only and must not introduce new semantic requirements.

use super::*;

fn pid(pid: Pid) -> EvolvablePid {
    EvolvablePid::from_filtered(pid, ProtectionLevel::Evolvable).unwrap()
}

fn node(pid: Pid) -> LifeNode {
    LifeNode {
        pid,
        parent: None,
        children: Vec::new(),
        birth_cycle: 0,
        death_cycle: None,
        death_reason: None,
        fitness_history: RingBuffer::new(64),
        genes: ProcessGenes { creativity: 1.0 },
        protection: ProtectionLevel::Evolvable,
    }
}

fn scheduler() -> PhiScheduler {
    let mut scheduler = PhiScheduler::default();
    for p in [1, 2, 3, 4] {
        scheduler.processes.insert(
            p,
            Process {
                pid: p,
                protection: ProtectionLevel::Evolvable,
            },
        );
        scheduler.life_graph.nodes.insert(p, node(p));
    }
    scheduler
}

fn policy() -> PopulationPolicy {
    PopulationPolicy {
        reproduction_budget: 12,
        max_offspring_per_parent: 4,
        min_population: 1,
    }
}

#[test]
fn tc_p01_o1a_eligible_capability_control() {
    assert!(EvolvablePid::from_filtered(1, ProtectionLevel::Evolvable).is_some());
    assert!(EvolvablePid::from_filtered(2, ProtectionLevel::Sandboxed).is_some());
}

#[test]
fn tc_n01_o1a_protected_and_immune_are_not_admitted() {
    assert!(EvolvablePid::from_filtered(1, ProtectionLevel::Protected).is_none());
    assert!(EvolvablePid::from_filtered(2, ProtectionLevel::Immune).is_none());
}

#[test]
fn tc_p02_o1c_valid_plan_reaches_build_success_control() {
    let mut s = scheduler();
    let plan = EvolutionPlan {
        population_generation: 0,
        deaths: Vec::new(),
        branches: vec![BranchRequest {
            parent: pid(1),
            mutation_rate: 0.1,
        }],
    };

    let result = s.apply_evolution_plan(
        plan,
        &policy(),
        &mut DefaultChildBuilder,
        &mut SeededPidAllocator::new(100),
    );

    assert!(result.is_ok());
}

#[test]
fn tc_p03_o1d_stale_request_does_not_invalidate_unrelated_valid_request() {
    let mut s = scheduler();
    let plan = EvolutionPlan {
        population_generation: 0,
        deaths: vec![DeathRequest {
            pid: pid(999),
            reason: DeathReason::Administrative,
        }],
        branches: vec![BranchRequest {
            parent: pid(1),
            mutation_rate: 0.1,
        }],
    };

    let result = s.apply_evolution_plan(
        plan,
        &policy(),
        &mut DefaultChildBuilder,
        &mut SeededPidAllocator::new(100),
    );

    assert!(result.is_ok());
    assert!(s.processes.contains_key(&100));
}

#[test]
fn tc_n02_o1b_structural_rejection_is_zero_mutation() {
    let mut s = scheduler();
    let before = s.clone();
    let plan = EvolutionPlan {
        population_generation: 0,
        deaths: vec![
            DeathRequest {
                pid: pid(1),
                reason: DeathReason::Administrative,
            },
            DeathRequest {
                pid: pid(1),
                reason: DeathReason::ResourceStarvation,
            },
        ],
        branches: vec![BranchRequest {
            parent: pid(2),
            mutation_rate: 0.1,
        }],
    };

    let result = s.apply_evolution_plan(
        plan,
        &policy(),
        &mut DefaultChildBuilder,
        &mut SeededPidAllocator::new(100),
    );

    assert!(matches!(
        result,
        Err(ApplyError::Structural(StructuralViolation::DuplicateDeathRequest { pid: 1 }))
    ));
    assert_eq!(s.processes, before.processes);
    assert_eq!(s.ready_queue, before.ready_queue);
    assert_eq!(s.generation, before.generation);
    assert_eq!(s.life_graph, before.life_graph);
    assert_eq!(s.audit, before.audit);
}

#[test]
fn tc_n03_o1c_build_failure_is_zero_commit() {
    let mut s = scheduler();
    let before = s.clone();
    let plan = EvolutionPlan {
        population_generation: 0,
        deaths: vec![DeathRequest {
            pid: pid(2),
            reason: DeathReason::Administrative,
        }],
        branches: vec![BranchRequest {
            parent: pid(1),
            mutation_rate: 0.1,
        }],
    };

    let result = s.apply_evolution_plan(
        plan,
        &policy(),
        &mut FailingBuilder { fail_at: 0 },
        &mut SeededPidAllocator::new(100),
    );

    assert!(matches!(
        result,
        Err(ApplyError::Build(BuildError::BuilderFailure { ordinal: 0 }))
    ));
    assert_eq!(s.processes, before.processes);
    assert_eq!(s.ready_queue, before.ready_queue);
    assert_eq!(s.generation, before.generation);
    assert_eq!(s.life_graph, before.life_graph);
    assert_eq!(s.audit, before.audit);
}

#[test]
fn tc_n04_o1d_stale_request_is_individually_tolerated() {
    let mut s = scheduler();
    let plan = EvolutionPlan {
        population_generation: 0,
        deaths: vec![DeathRequest {
            pid: pid(999),
            reason: DeathReason::Administrative,
        }],
        branches: Vec::new(),
    };

    let result = s.apply_evolution_plan(
        plan,
        &policy(),
        &mut DefaultChildBuilder,
        &mut SeededPidAllocator::new(100),
    );

    assert!(result.is_ok());
}

#[test]
fn tc_b01_o1b_invalid_plan_cannot_partially_apply_prior_mutation_opportunity() {
    let mut s = scheduler();
    let before = s.clone();
    let plan = EvolutionPlan {
        population_generation: 0,
        deaths: vec![
            DeathRequest {
                pid: pid(1),
                reason: DeathReason::Administrative,
            },
            DeathRequest {
                pid: pid(1),
                reason: DeathReason::ResourceStarvation,
            },
        ],
        branches: vec![BranchRequest {
            parent: pid(2),
            mutation_rate: 0.1,
        }],
    };

    let _ = s.apply_evolution_plan(
        plan,
        &policy(),
        &mut DefaultChildBuilder,
        &mut SeededPidAllocator::new(100),
    );

    assert_eq!(s.processes, before.processes);
    assert_eq!(s.ready_queue, before.ready_queue);
    assert_eq!(s.generation, before.generation);
    assert_eq!(s.life_graph, before.life_graph);
    assert_eq!(s.audit, before.audit);
}

#[test]
fn tc_b02_o1c_build_failure_cannot_commit_earlier_death() {
    let mut s = scheduler();
    let before = s.clone();
    let plan = EvolutionPlan {
        population_generation: 0,
        deaths: vec![DeathRequest {
            pid: pid(2),
            reason: DeathReason::Administrative,
        }],
        branches: vec![BranchRequest {
            parent: pid(1),
            mutation_rate: 0.1,
        }],
    };

    let _ = s.apply_evolution_plan(
        plan,
        &policy(),
        &mut FailingBuilder { fail_at: 0 },
        &mut SeededPidAllocator::new(100),
    );

    assert_eq!(s.processes, before.processes);
    assert_eq!(s.ready_queue, before.ready_queue);
    assert_eq!(s.generation, before.generation);
    assert_eq!(s.life_graph, before.life_graph);
    assert_eq!(s.audit, before.audit);
}

#[test]
fn tc_b03_o1d_stale_request_is_tolerated_before_or_after_valid_work() {
    // The plan model stores deaths and branches separately, so request ordering
    // inside one plan is not representable. Exercise the semantic ordering as
    // two real applications: stale then valid, and valid then stale.
    for stale_first in [true, false] {
        let mut s = scheduler();
        let stale = EvolutionPlan {
            population_generation: 0,
            deaths: vec![DeathRequest {
                pid: pid(999),
                reason: DeathReason::Administrative,
            }],
            branches: Vec::new(),
        };
        let valid = EvolutionPlan {
            population_generation: 0,
            deaths: Vec::new(),
            branches: vec![BranchRequest {
                parent: pid(1),
                mutation_rate: 0.1,
            }],
        };

        let (first, second) = if stale_first {
            (stale, valid)
        } else {
            (valid, stale)
        };

        let first_result = s.apply_evolution_plan(
            first,
            &policy(),
            &mut DefaultChildBuilder,
            &mut SeededPidAllocator::new(100),
        );
        assert!(first_result.is_ok());

        let second_result = s.apply_evolution_plan(
            second,
            &policy(),
            &mut DefaultChildBuilder,
            &mut SeededPidAllocator::new(101),
        );
        assert!(second_result.is_ok());
        assert!(s.processes.contains_key(&100) || s.processes.contains_key(&101));
    }
}

#[test]
fn tc_b04_o1a_structural_validity_does_not_bypass_capability_boundary() {
    let mut s = scheduler();
    let before = s.clone();
    assert!(EvolvablePid::from_filtered(1, ProtectionLevel::Protected).is_none());
    assert!(EvolvablePid::from_filtered(1, ProtectionLevel::Immune).is_none());
    assert_eq!(s.processes, before.processes);
}