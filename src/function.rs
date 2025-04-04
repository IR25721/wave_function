pub trait Trajectory<F>
where
    F: Fn(f32, f32) -> f32,
{
    pub fn get_trajectory(&self, values: Vec<f32>) -> Self;
}
