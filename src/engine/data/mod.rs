
struct QuadtreeNode {
    bounds: AABB,  // Axis-aligned bounding box
    lod_level: u32, // Level of Detail
    children: Option<[Box<QuadtreeNode>; 4]>, // Four child nodes
    mesh_data: Option<Mesh>, // Only leaf nodes contain mesh
}