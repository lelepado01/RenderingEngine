
#[derive(Debug)]
pub struct UnTexturedMaterial {
    pub ambient: [f32; 4],
    pub diffuse: [f32; 4],
    pub specular: [f32; 4],
    pub shininess: f32,
}

impl UnTexturedMaterial {
    pub fn from(
        mat : &tobj::Material, 
    ) -> UnTexturedMaterial {
    
        UnTexturedMaterial {
            ambient: [mat.ambient[0], mat.ambient[1], mat.ambient[2], 1.0],  
            diffuse : [mat.diffuse[0], mat.diffuse[1], mat.diffuse[2], 1.0],
            specular : [mat.specular[0], mat.specular[1], mat.specular[2], 1.0],
            shininess: mat.shininess,
        }
    }
}