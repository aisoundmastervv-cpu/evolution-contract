//! H2 fitness model.
//!
//! H1 is retained as a negative control in `H1_REJECTED.md`.
//! H2 separates genotype, phenotype, and evaluation:
//!
//!     efficiency gene -> observed CPU cost -> fitness
//!
//! The implementation below is deliberately mathematical. It does not claim
//! that a real process is causally affected by the gene; that is the pending
//! H3 experimental question.

const BASE_CPU_MS: f64 = 500.0;
const MAX_EFFICIENCY_EFFECT: f64 = 0.9;
const FITNESS_BUDGET_MS: f64 = 100.0;

/// Modelled CPU time for a fixed workload as a function of the efficiency gene.
pub fn observed_cpu_ms(efficiency: u8) -> f64 {
    BASE_CPU_MS * (1.0 - (efficiency as f64 / 255.0) * MAX_EFFICIENCY_EFFECT)
}

/// Fitness of an observed CPU cost under a fixed environmental budget.
pub fn evaluate_cpu_fitness(observed_cpu_ms: f64) -> f64 {
    FITNESS_BUDGET_MS / (FITNESS_BUDGET_MS + observed_cpu_ms)
}

/// Composite H2 fitness obtained by passing the gene through the phenotype model.
pub fn composite_fitness(efficiency: u8) -> f64 {
    evaluate_cpu_fitness(observed_cpu_ms(efficiency))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observed_cpu_monotonically_decreases() {
        for efficiency in 0..255 {
            assert!(
                observed_cpu_ms(efficiency + 1) < observed_cpu_ms(efficiency),
                "efficiency {} must reduce observed CPU time",
                efficiency
            );
        }
    }

    #[test]
    fn fitness_monotonically_decreases_with_cpu() {
        let mut previous = evaluate_cpu_fitness(10.0);
        for cpu in 11..=1000 {
            let current = evaluate_cpu_fitness(cpu as f64);
            assert!(current < previous, "fitness must decrease as CPU increases");
            previous = current;
        }
    }

    #[test]
    fn composite_fitness_monotonically_increases() {
        for efficiency in 0..255 {
            assert!(
                composite_fitness(efficiency + 1) > composite_fitness(efficiency),
                "fitness must increase from efficiency {} to {}",
                efficiency,
                efficiency + 1
            );
        }
    }

    #[test]
    fn pairwise_eff_plus_one_always_wins() {
        for efficiency in 0..255 {
            let low = composite_fitness(efficiency);
            let high = composite_fitness(efficiency + 1);
            assert!(high > low, "eff+1 must beat eff at {}", efficiency);
        }
    }

    #[test]
    fn boundaries_are_sensible() {
        let min = composite_fitness(0);
        let max = composite_fitness(255);
        assert!(min > 0.0 && min < 1.0);
        assert!(max > 0.0 && max < 1.0);
        assert!(max > min);
    }

    #[test]
    fn high_efficiency_beats_low_efficiency() {
        assert!(composite_fitness(255) > composite_fitness(0));
    }
}
