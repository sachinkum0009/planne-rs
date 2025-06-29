use std::time::Instant;

use planners::{
    config::Robot, joint_interpolation_planner::JointInterpolation, planner::MotionPlanner,
};

pub fn test_joint_interpolation() -> Result<(), String> {
    let start = vec![0.0, 0.5, 1.0, 1.57, 3.14, 0.5];
    let start_position = Robot::<3>::new([0.0, 0.0, 0.0]);
    let joints = start_position.get_joints();
    for joint in joints.iter() {
        println!("{}", joint);
    }

    let goal = vec![1.0, 1.0, 0.0, 0.0, 1.0, 1.0];
    let joint_inter_planner = JointInterpolation::new(start, goal);
    let now = Instant::now();
    let plan = joint_inter_planner.plan(100000);
    let elapsed = now.elapsed();
    println!("Planning took {} ms", elapsed.as_millis());
    // println!("{:?}", plan);
    Ok(())
}

fn main() -> Result<(), String> {
    test_joint_interpolation()?;
    Ok(())
}
