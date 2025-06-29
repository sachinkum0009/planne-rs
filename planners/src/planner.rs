pub trait MotionPlanner {
    fn set_start(&mut self, start: Vec<f64>);
    fn set_goal(&mut self, goal: Vec<f64>);

    fn plan(&self, steps: usize) -> Vec<Vec<f64>>;
}