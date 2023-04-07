

use cgmath::{Point3, Vector3};

pub trait ToPoint3 {
    type Output;

    fn to_point3(self) -> Self::Output; 
}

impl ToPoint3 for Vector3<f32> {
    type Output = Point3<f32>;

    fn to_point3(self) -> Self::Output {
        cgmath::Point3::new(self.x, self.y, self.z)
    }
}
