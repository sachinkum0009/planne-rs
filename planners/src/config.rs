pub struct Robot<const N: usize> {
    joints: [f64; N],
}

impl<const N: usize> Robot<N> {
    pub fn new(joints: [f64; N]) -> Self {
        Self { joints }
    }

    pub fn print_joints(&self) {
        println!("{:?}", self.joints);
    }

    pub fn get_joints(&self) -> [f64; N] {
        self.joints
    }
}
