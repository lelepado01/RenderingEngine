
pub trait Add {
    type Output;

    fn add(self, rhs: Self) -> Self::Output;
}

impl Add for [f32; 4] {
    type Output = [f32; 4];

    fn add(self, rhs: [f32; 4]) -> [f32; 4] {
        let mut result = [0.0; 4];
        for i in 0..4 {
            result[i] = self[i] + rhs[i];
        }
        result
    }
}

impl Add for [f32; 3] {
    type Output = [f32; 3];

    fn add(self, rhs: [f32; 3]) -> [f32; 3] {
        let mut result = [0.0; 3];
        for i in 0..3 {
            result[i] = self[i] + rhs[i];
        }
        result
    }
}

pub trait ScalarDiv {
    type Output;

    fn scalar_div(self, rhs: f32) -> Self::Output;
}

impl ScalarDiv for [f32; 4] {
    type Output = [f32; 4];

    fn scalar_div(self, rhs: f32) -> [f32; 4] {
        let mut result = [0.0; 4];
        for i in 0..4 {
            result[i] = self[i] / rhs;
        }
        result
    }
}