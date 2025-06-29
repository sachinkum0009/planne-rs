use crate::planner::MotionPlanner;

pub struct RapidlyExploringRandomTree {
    start: Vec<f64>,
    goal: Vec<f64>,
    step_size: f64,
    goal_threshold: f64,
    max_iterations: usize,
}

impl RapidlyExploringRandomTree {
    pub fn new(start: Vec<f64>, goal: Vec<f64>) -> Self {
        Self {
            start,
            goal,
            step_size: 0.1,
            goal_threshold: 0.2,
            max_iterations: 1000,
        }
    }

    pub fn set_step_size(&mut self, step_size: f64) {
        self.step_size = step_size;
    }
    pub fn set_goal_threshold(&mut self, goal_threshold: f64) {
        self.goal_threshold = goal_threshold;
    }
    pub fn set_max_iterations(&mut self, max_interations: usize) {
        self.max_iterations = max_interations;
    }
}

impl MotionPlanner for RapidlyExploringRandomTree {
    fn plan(&self, steps: usize) -> Vec<Vec<f64>> {
        let mut path = Vec::new();
        
        path
    }

    fn set_goal(&mut self, goal: Vec<f64>) {
        self.goal = goal;
    }

    fn set_start(&mut self, start: Vec<f64>) {
        self.start = start;
    }
}
