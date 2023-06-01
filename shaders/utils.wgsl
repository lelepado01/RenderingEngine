
struct InstancedVertexInput {
    @location(0) v_position: vec4<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) tex_coords: vec2<f32>,
}

struct TerrainVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) original_position: vec3<f32>,
    @location(2) material_id: f32,
};