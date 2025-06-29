use rayon::prelude::*;

use crate::planner::MotionPlanner;
use crate::config::Robot;

pub struct JointInterpolation {
    start: Vec<f64>,
    goal: Vec<f64>,
}

impl JointInterpolation {
    pub fn new(start: Vec<f64>, goal: Vec<f64>) -> Self {
        Self { start, goal }
    }
}

impl MotionPlanner for JointInterpolation {
    fn plan(&self, steps: usize) -> Vec<Vec<f64>> {
        let steps_f64 = steps as f64;
        let dim = self.start.len();

        // (0..=steps) is not directly parallel, so use (0..=steps).into_par_iter()
        (0..=steps)
            .into_par_iter()
            .map(|step| {
                let t = step as f64 / steps_f64;
                let mut interp = Vec::with_capacity(dim);
                for i in 0..dim {
                    interp.push(self.start[i] + (self.goal[i] - self.start[i]) * t);
                }
                interp
            })
            .collect()
    }
    fn set_goal(&mut self, goal: Vec<f64>) {
        self.goal = goal;
    }

    fn set_start(&mut self, start: Vec<f64>) {
        self.start = start;
    }
}
