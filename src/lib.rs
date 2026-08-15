use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Duration;

pub type Pid = u64;

#[derive(Clone, Debug, PartialEq)]
pub struct Phenotype {
    pub cpu_time_used: Duration,
    pub cpu_time_allotted: Duration,
    pub io_ops: u64,
    pub io_errors: u64,
    pub ipc_messages: u64,
    pub ipc_failures: u64,
    pub page_faults: u64,
    pub mean_latency: Duration,
    pub p99_latency: Duration,
    pub window_start_cycle: u64,
    pub window_end_cycle: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FitnessSnapshot {
    pub absolute: f32,
    pub relative_z: f32,
    pub percentile: f32,
    pub selection_score: f32,
    pub evaluated_at_cycle: u64,
    pub sample_size: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtectionLevel {
    Immune,
    Protected,
    Evolvable,
    Sandboxed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeathReason {
    ResourceStarvation,
    PersistentLowFitness { cycles_below_threshold: u64 },
    FaultAccumulation { fault_count: u64 },
    StaleProcess { idle_cycles: u64 },
    ParentTermination,
    Administrative,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProcessGenes {
    pub creativity: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LifeNode {
    pub pid: Pid,
    pub parent: Option<Pid>,
    pub children: Vec<Pid>,
    pub birth_cycle: u64,
    pub death_cycle: Option<u64>,
    pub death_reason: Option<DeathReason>,
    pub fitness_history: RingBuffer<FitnessSnapshot>,
    pub genes: ProcessGenes,
    pub protection: ProtectionLevel,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RingBuffer<T> {
    capacity: usize,
    values: VecDeque<T>,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            capacity,
            values: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: T) {
        if self.values.len() == self.capacity {
            self.values.pop_front();
        }
        self.values.push_back(value);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LifeGraph {
    pub nodes: HashMap<Pid, LifeNode>,
    pub fossils: Vec<LifeNode>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EvolvablePid(Pid);

impl EvolvablePid {
    pub(crate) fn from_filtered(pid: Pid, level: ProtectionLevel) -> Option<Self> {
        matches!(
            level,
            ProtectionLevel::Evolvable | ProtectionLevel::Sandboxed
        )
        .then_some(Self(pid))
    }

    pub fn get(self) -> Pid {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EvolutionEvent {
    Survive {
        pid: EvolvablePid,
    },
    Branch {
        parent: EvolvablePid,
        mutation_rate: f32,
    },
    Die {
        pid: EvolvablePid,
        reason: DeathReason,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeathRequest {
    pub pid: EvolvablePid,
    pub reason: DeathReason,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BranchRequest {
    pub parent: EvolvablePid,
    pub mutation_rate: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvolutionPlan {
    pub population_generation: u64,
    pub deaths: Vec<DeathRequest>,
    pub branches: Vec<BranchRequest>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StructuralViolation {
    DuplicateDeathRequest {
        pid: Pid,
    },
    ConflictingRequest {
        pid: Pid,
    },
    ReproductionBudgetExceeded {
        requested: usize,
        budget: usize,
    },
    ParentOffspringCapExceeded {
        parent: Pid,
        requested: usize,
        cap: usize,
    },
    InvalidMutationRate {
        rate: f32,
        parent: Pid,
    },
    PopulationFloorViolated {
        after_deaths: usize,
        min_population: usize,
    },
    InvalidPopulationGeneration {
        expected: u64,
        actual: u64,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuildError {
    PidCollision { pid: Pid },
    ResourceUnavailable { ordinal: usize },
    BuilderFailure { ordinal: usize },
}

#[derive(Clone, Debug, PartialEq)]
pub enum ApplyError {
    Structural(StructuralViolation),
    Build(BuildError),
    PidCollisionAtCommit { pid: Pid },
}

#[derive(Clone, Debug, PartialEq)]
pub enum AuditEvent {
    Applied {
        generation_before: u64,
        generation_after: u64,
    },
    Rejected {
        reason: StructuralViolation,
    },
    PartialApply {
        skipped_deaths: usize,
        skipped_branches: usize,
    },
}

pub trait PidAllocator {
    fn allocate(&mut self) -> Pid;
}

#[derive(Clone, Debug)]
pub struct SeededPidAllocator {
    next: Pid,
}

impl SeededPidAllocator {
    pub fn new(seed: Pid) -> Self {
        Self { next: seed }
    }
}

impl PidAllocator for SeededPidAllocator {
    fn allocate(&mut self) -> Pid {
        let pid = self.next;
        self.next = self.next.wrapping_add(1);
        pid
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LifeNodeDraft {
    pub pid: Pid,
    pub parent: Pid,
    pub genes: ProcessGenes,
    pub mutation_rate: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PendingChild {
    pub draft: LifeNodeDraft,
}

pub trait ChildBuilder {
    fn build_child(
        &mut self,
        parent: &LifeNode,
        pid: Pid,
        mutation_rate: f32,
        ordinal: usize,
    ) -> Result<PendingChild, BuildError>;
}

#[derive(Clone, Debug, Default)]
pub struct DefaultChildBuilder;

impl ChildBuilder for DefaultChildBuilder {
    fn build_child(
        &mut self,
        parent: &LifeNode,
        pid: Pid,
        mutation_rate: f32,
        _ordinal: usize,
    ) -> Result<PendingChild, BuildError> {
        Ok(PendingChild {
            draft: LifeNodeDraft {
                pid,
                parent: parent.pid,
                genes: parent.genes.clone(),
                mutation_rate,
            },
        })
    }
}

#[derive(Clone, Debug)]
pub struct FailingBuilder {
    pub fail_at: usize,
}

impl ChildBuilder for FailingBuilder {
    fn build_child(
        &mut self,
        parent: &LifeNode,
        pid: Pid,
        mutation_rate: f32,
        ordinal: usize,
    ) -> Result<PendingChild, BuildError> {
        if ordinal == self.fail_at {
            return Err(BuildError::BuilderFailure { ordinal });
        }
        DefaultChildBuilder.build_child(parent, pid, mutation_rate, ordinal)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PopulationPolicy {
    pub reproduction_budget: usize,
    pub max_offspring_per_parent: usize,
    pub min_population: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Process {
    pub pid: Pid,
    pub protection: ProtectionLevel,
}

#[derive(Clone, Debug, Default)]
pub struct PhiScheduler {
    pub processes: HashMap<Pid, Process>,
    pub ready_queue: Vec<Pid>,
    pub generation: u64,
    pub life_graph: LifeGraph,
    pub audit: Vec<AuditEvent>,
}

impl PhiScheduler {
    pub fn next_runnable(&mut self) -> Option<Pid> {
        while let Some(pid) = self.ready_queue.first().copied() {
            self.ready_queue.remove(0);
            if self.processes.contains_key(&pid) {
                return Some(pid);
            }
        }
        None
    }

    pub fn apply_evolution_plan<B: ChildBuilder, A: PidAllocator>(
        &mut self,
        plan: EvolutionPlan,
        policy: &PopulationPolicy,
        builder: &mut B,
        allocator: &mut A,
    ) -> Result<(), ApplyError> {
        self.validate_plan(&plan, policy)?;

        // Temporal staleness is resolved per request: missing PIDs are skipped,
        // rather than invalidating unrelated requests in the same plan.
        let deaths: Vec<_> = plan
            .deaths
            .into_iter()
            .filter(|r| self.processes.contains_key(&r.pid.get()))
            .collect();
        let branches: Vec<_> = plan
            .branches
            .into_iter()
            .filter(|r| self.processes.contains_key(&r.parent.get()))
            .collect();

        // Build-then-commit: no structural mutation occurs while children are built.
        let mut pending = Vec::with_capacity(branches.len());
        let mut pending_pids = HashSet::with_capacity(branches.len());
        for (ordinal, request) in branches.iter().enumerate() {
            let parent_pid = request.parent.get();
            let parent = self
                .life_graph
                .nodes
                .get(&parent_pid)
                .expect("active evolvable process must have a LifeNode");
            let pid = allocator.allocate();
            if self.processes.contains_key(&pid) || !pending_pids.insert(pid) {
                return Err(ApplyError::Build(BuildError::PidCollision { pid }));
            }
            let child = builder
                .build_child(parent, pid, request.mutation_rate, ordinal)
                .map_err(ApplyError::Build)?;
            pending.push(child);
        }

        let generation_before = self.generation;

        // Commit phase. HashMap insertion is treated as infallible except for the
        // explicit collision guard above. Deaths and children become visible here.
        for request in deaths {
            let pid = request.pid.get();
            self.processes.remove(&pid);
            if let Some(mut node) = self.life_graph.nodes.remove(&pid) {
                node.death_cycle = Some(self.generation);
                node.death_reason = Some(request.reason);
                self.life_graph.fossils.push(node);
            }
        }

        for child in pending {
            let d = child.draft;
            if self.processes.contains_key(&d.pid) {
                return Err(ApplyError::PidCollisionAtCommit { pid: d.pid });
            }
            self.processes.insert(
                d.pid,
                Process {
                    pid: d.pid,
                    protection: ProtectionLevel::Evolvable,
                },
            );
            self.life_graph.nodes.insert(
                d.pid,
                LifeNode {
                    pid: d.pid,
                    parent: Some(d.parent),
                    children: Vec::new(),
                    birth_cycle: self.generation,
                    death_cycle: None,
                    death_reason: None,
                    fitness_history: RingBuffer::new(64),
                    genes: d.genes,
                    protection: ProtectionLevel::Evolvable,
                },
            );
            if let Some(parent) = self.life_graph.nodes.get_mut(&d.parent) {
                parent.children.push(d.pid);
            }
            self.ready_queue.push(d.pid);
        }

        self.generation = self.generation.wrapping_add(1);
        self.audit.push(AuditEvent::Applied {
            generation_before,
            generation_after: self.generation,
        });
        Ok(())
    }

    fn validate_plan(
        &self,
        plan: &EvolutionPlan,
        policy: &PopulationPolicy,
    ) -> Result<(), ApplyError> {
        if plan.population_generation != self.generation {
            return Err(ApplyError::Structural(
                StructuralViolation::InvalidPopulationGeneration {
                    expected: self.generation,
                    actual: plan.population_generation,
                },
            ));
        }

        let mut death_ids = HashSet::with_capacity(plan.deaths.len());
        for death in &plan.deaths {
            if !death_ids.insert(death.pid.get()) {
                return Err(ApplyError::Structural(
                    StructuralViolation::DuplicateDeathRequest {
                        pid: death.pid.get(),
                    },
                ));
            }
        }

        let mut branch_parents = HashMap::<Pid, usize>::new();
        for branch in &plan.branches {
            if !branch.mutation_rate.is_finite() || !(0.0..=1.0).contains(&branch.mutation_rate) {
                return Err(ApplyError::Structural(
                    StructuralViolation::InvalidMutationRate {
                        rate: branch.mutation_rate,
                        parent: branch.parent.get(),
                    },
                ));
            }
            *branch_parents.entry(branch.parent.get()).or_default() += 1;
        }

        for (parent, requested) in branch_parents {
            if requested > policy.max_offspring_per_parent {
                return Err(ApplyError::Structural(
                    StructuralViolation::ParentOffspringCapExceeded {
                        parent,
                        requested,
                        cap: policy.max_offspring_per_parent,
                    },
                ));
            }
        }

        if plan.branches.len() > policy.reproduction_budget {
            return Err(ApplyError::Structural(
                StructuralViolation::ReproductionBudgetExceeded {
                    requested: plan.branches.len(),
                    budget: policy.reproduction_budget,
                },
            ));
        }

        let death_ids: HashSet<_> = plan.deaths.iter().map(|d| d.pid.get()).collect();
        let branch_parents: HashSet<_> = plan.branches.iter().map(|b| b.parent.get()).collect();
        if let Some(pid) = death_ids.intersection(&branch_parents).next() {
            return Err(ApplyError::Structural(
                StructuralViolation::ConflictingRequest { pid: *pid },
            ));
        }

        let after_deaths = self.processes.len().saturating_sub(death_ids.len());
        if after_deaths < policy.min_population {
            return Err(ApplyError::Structural(
                StructuralViolation::PopulationFloorViolated {
                    after_deaths,
                    min_population: policy.min_population,
                },
            ));
        }

        Ok(())
    }
}

pub trait FitnessEvaluator {
    fn evaluate_absolute(&self, phenotype: &Phenotype, weights: &FitnessWeights) -> f32;

    fn evaluate_relative(&self, absolutes: &[f32], target_index: usize) -> (f32, f32);
}

#[derive(Clone, Debug, PartialEq)]
pub struct FitnessWeights {
    pub cpu: f32,
    pub io_errors: f32,
    pub ipc_failures: f32,
    pub page_faults: f32,
    pub latency: f32,
}

#[cfg(test)]
mod tests {
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
    fn evolvable_pid_witness_rejects_protected_and_immune() {
        assert!(EvolvablePid::from_filtered(1, ProtectionLevel::Protected).is_none());
        assert!(EvolvablePid::from_filtered(2, ProtectionLevel::Immune).is_none());
        assert!(EvolvablePid::from_filtered(3, ProtectionLevel::Evolvable).is_some());
        assert!(EvolvablePid::from_filtered(4, ProtectionLevel::Sandboxed).is_some());
    }

    #[test]
    fn conflicting_request_rejects_whole_plan() {
        let mut s = scheduler();
        let before = s.clone();
        let plan = EvolutionPlan {
            population_generation: 0,
            deaths: vec![DeathRequest {
                pid: pid(1),
                reason: DeathReason::Administrative,
            }],
            branches: vec![BranchRequest {
                parent: pid(1),
                mutation_rate: 0.1,
            }],
        };
        let err = s
            .apply_evolution_plan(
                plan,
                &policy(),
                &mut DefaultChildBuilder,
                &mut SeededPidAllocator::new(100),
            )
            .unwrap_err();
        assert!(matches!(
            err,
            ApplyError::Structural(StructuralViolation::ConflictingRequest { pid: 1 })
        ));
        assert_eq!(s.processes, before.processes);
        assert_eq!(s.generation, before.generation);
        assert_eq!(s.life_graph, before.life_graph);
    }

    #[test]
    fn duplicate_death_request_is_structural_and_rejects_whole_plan() {
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
            branches: Vec::new(),
        };
        let err = s
            .apply_evolution_plan(
                plan,
                &policy(),
                &mut DefaultChildBuilder,
                &mut SeededPidAllocator::new(100),
            )
            .unwrap_err();
        assert!(matches!(
            err,
            ApplyError::Structural(StructuralViolation::DuplicateDeathRequest { pid: 1 })
        ));
        assert_eq!(s.processes, before.processes);
        assert_eq!(s.generation, before.generation);
        assert_eq!(s.life_graph, before.life_graph);
    }

    #[test]
    fn budget_overflow_is_structural() {
        let mut s = scheduler();
        let mut p = policy();
        p.reproduction_budget = 1;
        let plan = EvolutionPlan {
            population_generation: 0,
            deaths: Vec::new(),
            branches: vec![
                BranchRequest {
                    parent: pid(1),
                    mutation_rate: 0.1,
                },
                BranchRequest {
                    parent: pid(2),
                    mutation_rate: 0.1,
                },
            ],
        };
        let err = s
            .apply_evolution_plan(
                plan,
                &p,
                &mut DefaultChildBuilder,
                &mut SeededPidAllocator::new(100),
            )
            .unwrap_err();
        assert!(matches!(
            err,
            ApplyError::Structural(StructuralViolation::ReproductionBudgetExceeded {
                requested: 2,
                budget: 1
            })
        ));
    }

    #[test]
    fn per_parent_cap_is_structural() {
        let mut s = scheduler();
        let mut p = policy();
        p.max_offspring_per_parent = 2;
        let plan = EvolutionPlan {
            population_generation: 0,
            deaths: Vec::new(),
            branches: vec![
                BranchRequest {
                    parent: pid(1),
                    mutation_rate: 0.1,
                },
                BranchRequest {
                    parent: pid(1),
                    mutation_rate: 0.2,
                },
                BranchRequest {
                    parent: pid(1),
                    mutation_rate: 0.3,
                },
            ],
        };
        let err = s
            .apply_evolution_plan(
                plan,
                &p,
                &mut DefaultChildBuilder,
                &mut SeededPidAllocator::new(100),
            )
            .unwrap_err();
        assert!(matches!(
            err,
            ApplyError::Structural(StructuralViolation::ParentOffspringCapExceeded {
                parent: 1,
                requested: 3,
                cap: 2
            })
        ));
    }

    #[test]
    fn stale_parent_is_partial_apply() {
        let mut s = scheduler();
        s.processes.remove(&2);
        let plan = EvolutionPlan {
            population_generation: 0,
            deaths: vec![
                DeathRequest {
                    pid: pid(1),
                    reason: DeathReason::Administrative,
                },
                DeathRequest {
                    pid: pid(99),
                    reason: DeathReason::Administrative,
                },
            ],
            branches: vec![
                BranchRequest {
                    parent: pid(2),
                    mutation_rate: 0.1,
                },
                BranchRequest {
                    parent: pid(3),
                    mutation_rate: 0.1,
                },
            ],
        };
        let result = s.apply_evolution_plan(
            plan,
            &policy(),
            &mut DefaultChildBuilder,
            &mut SeededPidAllocator::new(100),
        );
        assert!(result.is_ok());
        assert!(!s.processes.contains_key(&1));
        assert!(s.processes.contains_key(&3));
        assert!(s.processes.contains_key(&100));
    }

    #[test]
    fn population_floor_rejects_plan() {
        let mut s = scheduler();
        let mut p = policy();
        p.min_population = 4;
        let plan = EvolutionPlan {
            population_generation: 0,
            deaths: vec![DeathRequest {
                pid: pid(1),
                reason: DeathReason::Administrative,
            }],
            branches: Vec::new(),
        };
        let err = s
            .apply_evolution_plan(
                plan,
                &p,
                &mut DefaultChildBuilder,
                &mut SeededPidAllocator::new(100),
            )
            .unwrap_err();
        assert!(matches!(
            err,
            ApplyError::Structural(StructuralViolation::PopulationFloorViolated {
                after_deaths: 3,
                min_population: 4
            })
        ));
    }

    #[test]
    fn failed_build_commits_nothing() {
        let mut s = scheduler();
        let before = s.clone();
        let plan = EvolutionPlan {
            population_generation: 0,
            deaths: vec![DeathRequest {
                pid: pid(1),
                reason: DeathReason::Administrative,
            }],
            branches: (0..12)
                .map(|i| BranchRequest {
                    parent: pid(2 + (i % 2) as u64),
                    mutation_rate: 0.1,
                })
                .collect(),
        };
        let err = s
            .apply_evolution_plan(
                plan,
                &policy(),
                &mut FailingBuilder { fail_at: 7 },
                &mut SeededPidAllocator::new(100),
            )
            .unwrap_err();
        assert!(matches!(
            err,
            ApplyError::Build(BuildError::BuilderFailure { ordinal: 7 })
        ));
        assert_eq!(s.processes, before.processes);
        assert_eq!(s.generation, before.generation);
        assert_eq!(s.life_graph, before.life_graph);
    }

    #[test]
    fn lazy_invalidation_skips_dead_pid() {
        let mut s = scheduler();
        s.ready_queue = vec![1, 99, 2];
        s.processes.remove(&1);
        assert_eq!(s.next_runnable(), Some(2));
    }

    #[test]
    fn generation_mismatch_is_structural_rejection() {
        let mut s = scheduler();
        let before = s.clone();
        let plan = EvolutionPlan {
            population_generation: 42,
            deaths: Vec::new(),
            branches: Vec::new(),
        };
        let err = s
            .apply_evolution_plan(
                plan,
                &policy(),
                &mut DefaultChildBuilder,
                &mut SeededPidAllocator::new(100),
            )
            .unwrap_err();
        assert!(matches!(
            err,
            ApplyError::Structural(StructuralViolation::InvalidPopulationGeneration {
                expected: 0,
                actual: 42
            })
        ));
        assert_eq!(s.processes, before.processes);
        assert_eq!(s.generation, before.generation);
        assert_eq!(s.life_graph, before.life_graph);
    }
}
