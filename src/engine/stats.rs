
pub struct EngineStats {
    pub fps : f32,
    pub frames_render_time : f32,
    pub frames_draw_calls : usize,

    pub instances_drawn : usize,

    pub bytes_to_gpu : usize,
}


impl EngineStats {
    pub fn new() -> Self {
        Self {
            fps : 0.0,
            frames_draw_calls : 0,
            frames_render_time : 0.0,

            instances_drawn : 0,

            bytes_to_gpu : 0,
        }
    }

    pub fn update(&mut self, delta_time : f32) {
        self.frames_render_time = delta_time;
        self.fps =  1.0 / delta_time;
        self.bytes_to_gpu = 0;
    }
}