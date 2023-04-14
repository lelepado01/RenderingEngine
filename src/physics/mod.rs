
pub fn get_distance(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    (a[0] - b[0]).abs() + (a[2] - b[2]).abs()
}