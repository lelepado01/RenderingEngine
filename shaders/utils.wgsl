
struct VertexInput {
    @location(0) v_position: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
}

struct InstanceInput {
    @location(3) i_position: vec4<f32>,
    @location(4) material_id: f32,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) original_position: vec3<f32>,
    @location(3) instance_material_id: f32,
};