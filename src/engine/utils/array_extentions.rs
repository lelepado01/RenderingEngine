

pub trait ToArray4 {
    type Output;

    fn to_arr4(self) -> Self::Output;
}

impl ToArray4 for [f32; 3] {
    type Output = [f32; 4];

    fn to_arr4(self) -> [f32; 4] {
        let mut result = [0.0; 4];
        for i in 0..3 {
            result[i] = self[i];
        }
        result
    }
}