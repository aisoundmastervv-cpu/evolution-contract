//! LifeGraph harness: observes semantic projections and evaluates frozen C-LG/O-LG only.
//! Test-only. It must not introduce new semantic requirements.

use super::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
struct ActiveProcessProjection {
    pids: Vec<Pid>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ActiveLifeGraphNodeProjection {
    pid: Pid,
    parent: Option<Pid>,
    children: Vec<Pid>,
    birth_cycle: u64,
    death_cycle: Option<u64>,
    death_reason: Option<DeathReason>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ActiveLifeGraphProjection {
    nodes: Vec<ActiveLifeGraphNodeProjection>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct HistoricalNodeProjection {
    pid: Pid,
    parent: Option<Pid>,
    children: Vec<Pid>,
    birth_cycle: u64,
    death_cycle: Option<u64>,
    death_reason: Option<DeathReason>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct HistoricalProjection {
    fossils: Vec<HistoricalNodeProjection>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SemanticProjection {
    active_processes: ActiveProcessProjection,
    active_life_graph: ActiveLifeGraphProjection,
    history: HistoricalProjection,
    ready_queue: Vec<Pid>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScenarioKind {
    Stable,
    Birth,
    Death,
}

fn observe(scheduler: &PhiScheduler) -> SemanticProjection {
    let mut active_processes: Vec<_> = scheduler.processes.keys().copied().collect();
    active_processes.sort_unstable();

    let mut nodes: Vec<_> = scheduler
        .life_graph
        .nodes
        .values()
        .map(|node| ActiveLifeGraphNodeProjection {
            pid: node.pid,
            parent: node.parent,
            children: {
                let mut children = node.children.clone();
                children.sort_unstable();
                children
            },
            birth_cycle: node.birth_cycle,
            death_cycle: node.death_cycle,
            death_reason: node.death_reason.clone(),
        })
        .collect();
    nodes.sort_by_key(|node| node.pid);

    let mut fossils: Vec<_> = scheduler
        .life_graph
        .fossils
        .iter()
        .map(|node| HistoricalNodeProjection {
            pid: node.pid,
            parent: node.parent,
            children: {
                let mut children = node.children.clone();
                children.sort_unstable();
                children
            },
            birth_cycle: node.birth_cycle,
            death_cycle: node.death_cycle,
            death_reason: node.death_reason.clone(),
        })
        .collect();
    fossils.sort_by_key(|node| node.pid);

    SemanticProjection {
        active_processes: ActiveProcessProjection { pids: active_processes },
        active_life_graph: ActiveLifeGraphProjection { nodes },
        history: HistoricalProjection { fossils },
        ready_queue: scheduler.ready_queue.clone(),
    }
}

fn active_node_map(
    projection: &ActiveLifeGraphProjection,
) -> BTreeMap<Pid, &ActiveLifeGraphNodeProjection> {
    projection.nodes.iter().map(|node| (node.pid, node)).collect()
}

fn evaluate_active_correspondence(projection: &SemanticProjection) -> Result<(), String> {
    let nodes = active_node_map(&projection.active_life_graph);
    for pid in &projection.active_processes.pids {
        if !nodes.contains_key(pid) {
            return Err(format!("active process {pid} has no required active LifeGraph node"));
        }
    }
    Ok(())
}

fn evaluate_birth(
    after: &SemanticProjection,
    child_pid: Pid,
    parent_pid: Pid,
) -> Result<(), String> {
    evaluate_active_correspondence(after)?;
    let nodes = active_node_map(&after.active_life_graph);
    let child = nodes
        .get(&child_pid)
        .ok_or_else(|| format!("birth child {child_pid} has no active LifeGraph node"))?;

    if child.parent != Some(parent_pid) {
        return Err(format!("birth child {child_pid} has incorrect parent relation"));
    }

    let parent = nodes
        .get(&parent_pid)
        .ok_or_else(|| format!("birth parent {parent_pid} has no active LifeGraph node"))?;
    if !parent.children.contains(&child_pid) {
        return Err(format!("parent {parent_pid} lacks child relation to {child_pid}"));
    }

    if !after.ready_queue.contains(&child_pid) {
        return Err(format!("birth child {child_pid} is not eligible for scheduling"));
    }

    Ok(())
}

fn evaluate_death(
    after: &SemanticProjection,
    former_pid: Pid,
) -> Result<(), String> {
    evaluate_active_correspondence(after)?;
    let nodes = active_node_map(&after.active_life_graph);
    if nodes.contains_key(&former_pid) {
        return Err(format!("dead process {former_pid} still has an active LifeGraph node"));
    }

    let fossil = after
        .history
        .fossils
        .iter()
        .find(|node| node.pid == former_pid)
        .ok_or_else(|| format!("dead process {former_pid} has no historical representation"))?;

    if fossil.death_cycle.is_none() {
        return Err(format!("dead process {former_pid} has no death metadata"));
    }
    if fossil.death_reason.is_none() {
        return Err(format!("dead process {former_pid} has no death reason metadata"));
    }

    Ok(())
}

fn scenario_is_independent(kind: ScenarioKind) -> bool {
    matches!(kind, ScenarioKind::Stable | ScenarioKind::Birth | ScenarioKind::Death)
}

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
    for p in [1, 2, 3] {
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
        reproduction_budget: 4,
        max_offspring_per_parent: 2,
        min_population: 1,
    }
}

#[test]
fn lg_p01_stable_active_correspondence() {
    assert!(scenario_is_independent(ScenarioKind::Stable));
    let mut scheduler = scheduler();
    let before = observe(&scheduler);

    let result = scheduler.apply_evolution_plan(
        EvolutionPlan {
            population_generation: 0,
            deaths: Vec::new(),
            branches: Vec::new(),
        },
        &policy(),
        &mut DefaultChildBuilder,
        &mut SeededPidAllocator::new(100),
    );
    assert!(result.is_ok());

    let after = observe(&scheduler);
    assert_eq!(before.active_processes, after.active_processes);
    assert!(evaluate_active_correspondence(&after).is_ok());
}

#[test]
fn lg_p02_accepted_birth_has_required_life_graph_semantics() {
    assert!(scenario_is_independent(ScenarioKind::Birth));
    let mut scheduler = scheduler();
    let child_pid = 100;

    let result = scheduler.apply_evolution_plan(
        EvolutionPlan {
            population_generation: 0,
            deaths: Vec::new(),
            branches: vec![BranchRequest {
                parent: pid(1),
                mutation_rate: 0.1,
            }],
        },
        &policy(),
        &mut DefaultChildBuilder,
        &mut SeededPidAllocator::new(child_pid),
    );
    assert!(result.is_ok());

    let after = observe(&scheduler);
    assert!(evaluate_birth(&after, child_pid, 1).is_ok());
}

#[test]
fn lg_p03_accepted_death_has_required_life_graph_semantics() {
    assert!(scenario_is_independent(ScenarioKind::Death));
    let mut scheduler = scheduler();
    let result = scheduler.apply_evolution_plan(
        EvolutionPlan {
            population_generation: 0,
            deaths: vec![DeathRequest {
                pid: pid(3),
                reason: DeathReason::Administrative,
            }],
            branches: Vec::new(),
        },
        &policy(),
        &mut DefaultChildBuilder,
        &mut SeededPidAllocator::new(100),
    );
    assert!(result.is_ok());

    let after = observe(&scheduler);
    assert!(evaluate_death(&after, 3).is_ok());
}

#[test]
fn lg_n01_missing_active_lifegraph_node_is_rejected() {
    let mut scheduler = scheduler();
    scheduler.life_graph.nodes.remove(&2);
    let projection = observe(&scheduler);

    assert!(evaluate_active_correspondence(&projection).is_err());
}

#[test]
fn lg_n02_birth_missing_child_node_is_rejected() {
    let mut scheduler = scheduler();
    let result = scheduler.apply_evolution_plan(
        EvolutionPlan {
            population_generation: 0,
            deaths: Vec::new(),
            branches: vec![BranchRequest {
                parent: pid(1),
                mutation_rate: 0.1,
            }],
        },
        &policy(),
        &mut DefaultChildBuilder,
        &mut SeededPidAllocator::new(100),
    );
    assert!(result.is_ok());
    scheduler.life_graph.nodes.remove(&100);

    assert!(evaluate_birth(&observe(&scheduler), 100, 1).is_err());
}

#[test]
fn lg_n03a_birth_missing_parent_child_relation_is_rejected() {
    let mut scheduler = scheduler();
    let result = scheduler.apply_evolution_plan(
        EvolutionPlan {
            population_generation: 0,
            deaths: Vec::new(),
            branches: vec![BranchRequest {
                parent: pid(1),
                mutation_rate: 0.1,
            }],
        },
        &policy(),
        &mut DefaultChildBuilder,
        &mut SeededPidAllocator::new(100),
    );
    assert!(result.is_ok());
    scheduler.life_graph.nodes.get_mut(&1).unwrap().children.clear();

    assert!(evaluate_birth(&observe(&scheduler), 100, 1).is_err());
}

#[test]
fn lg_n04_death_leaves_active_node_is_rejected() {
    let mut scheduler = scheduler();
    scheduler.processes.remove(&3);
    let projection = observe(&scheduler);

    assert!(evaluate_death(&projection, 3).is_err());
}

#[test]
fn lg_n05a_death_missing_history_is_rejected() {
    let mut scheduler = scheduler();
    let result = scheduler.apply_evolution_plan(
        EvolutionPlan {
            population_generation: 0,
            deaths: vec![DeathRequest {
                pid: pid(3),
                reason: DeathReason::Administrative,
            }],
            branches: Vec::new(),
        },
        &policy(),
        &mut DefaultChildBuilder,
        &mut SeededPidAllocator::new(100),
    );
    assert!(result.is_ok());
    scheduler.life_graph.fossils.clear();

    assert!(evaluate_death(&observe(&scheduler), 3).is_err());
}

#[test]
fn lg_n05b_death_missing_death_metadata_is_rejected() {
    let mut scheduler = scheduler();
    let result = scheduler.apply_evolution_plan(
        EvolutionPlan {
            population_generation: 0,
            deaths: vec![DeathRequest {
                pid: pid(3),
                reason: DeathReason::Administrative,
            }],
            branches: Vec::new(),
        },
        &policy(),
        &mut DefaultChildBuilder,
        &mut SeededPidAllocator::new(100),
    );
    assert!(result.is_ok());
    let fossil = scheduler.life_graph.fossils.iter_mut().find(|n| n.pid == 3).unwrap();
    fossil.death_reason = None;

    assert!(evaluate_death(&observe(&scheduler), 3).is_err());
}
