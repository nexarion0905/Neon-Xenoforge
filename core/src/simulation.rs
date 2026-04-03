pub struct TemporalSimulationEngine {
    pub max_iterations: u32,
    pub current_iteration: u32,
}

impl TemporalSimulationEngine {
    pub fn new(max_iterations: u32) -> Self {
        TemporalSimulationEngine { max_iterations, current_iteration: 0, }
    }

    pub fn step(&mut self) -> Result<u32, String> {
        if self.current_iteration >= self.max_iterations {
            return Err("Max iterations reached".to_string());
        }
        self.current_iteration += 1;
        Ok(self.current_iteration)
    }

    pub fn run(&mut self, steps: u32) -> Result<Vec<u32>, String> {
        let mut results = Vec::new();
        for _ in 0..steps {
            results.push(self.step()?);
        }
        Ok(results)
    }

    pub fn reset(&mut self) {
        self.current_iteration = 0;
    }

    pub fn get_progress(&self) -> f64 {
        if self.max_iterations == 0 {
            return 0.0;
        }
        (self.current_iteration as f64 / self.max_iterations as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = TemporalSimulationEngine::new(100);
        assert_eq!(engine.max_iterations, 100);
        assert_eq!(engine.current_iteration, 0);
    }

    #[test]
    fn test_step() {
        let mut engine = TemporalSimulationEngine::new(100);
        let result = engine.step();
        assert!(result.is_ok());
        assert_eq!(engine.current_iteration, 1);
    }

    #[test]
    fn test_run() {
        let mut engine = TemporalSimulationEngine::new(100);
        let result = engine.run(5);
        assert!(result.is_ok());
        let iterations = result.unwrap();
        assert_eq!(iterations.len(), 5);
        assert_eq!(engine.current_iteration, 5);
    }

    #[test]
    fn test_max_iterations_exceeded() {
        let mut engine = TemporalSimulationEngine::new(2);
        engine.run(2).ok();
        let result = engine.step();
        assert!(result.is_err());
    }

    #[test]
    fn test_reset() {
        let mut engine = TemporalSimulationEngine::new(100);
        engine.run(5).ok();
        engine.reset();
        assert_eq!(engine.current_iteration, 0);
    }

    #[test]
    fn test_get_progress() {
        let mut engine = TemporalSimulationEngine::new(100);
        engine.run(50).ok();
        let progress = engine.get_progress();
        assert_eq!(progress, 50.0);
    }
}