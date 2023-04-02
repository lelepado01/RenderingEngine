
use image::{RgbaImage, GenericImageView};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureType {
    Texture2D,
    Texture3D,
    TextureDepth,
}

pub struct TextureBuilder {
    texture_type : TextureType,
    texture_dimensions : wgpu::TextureDimension,
    texture_size : wgpu::Extent3d,
    texture_format : wgpu::TextureFormat,
    texture_usage : wgpu::TextureUsages,
    pixel_data : RgbaImage,
    image_layout : wgpu::ImageDataLayout,
}

impl TextureBuilder {
    pub fn new(tex_path : &str, texture_type : TextureType) -> Self {

        if texture_type != TextureType::TextureDepth{
            let diffuse_bytes = std::fs::read(tex_path).expect("Failed to read texture file"); 
            let diffuse_image = image::load_from_memory(diffuse_bytes.as_slice()).unwrap();
            let diffuse_rgba = diffuse_image.to_rgba8();
            let dimensions = diffuse_image.dimensions();

            let image_layout = wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            };
    
            let texture_size = wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            };

            Self {
                texture_type : texture_type,
                texture_dimensions : wgpu::TextureDimension::D2,
                texture_format : wgpu::TextureFormat::Rgba8UnormSrgb,
                texture_usage : wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                pixel_data : diffuse_rgba,
                image_layout : image_layout,
                texture_size : texture_size,
            }
        } else {
            let size = wgpu::Extent3d { // 2.
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            };
            
            Self{
                texture_type : texture_type,
                texture_dimensions : wgpu::TextureDimension::D2,
                texture_format : wgpu::TextureFormat::Rgba8UnormSrgb,
                texture_usage : wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                pixel_data : RgbaImage::new(1, 1),
                image_layout : wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(4 * 1),
                    rows_per_image: std::num::NonZeroU32::new(1),
                },
                texture_size : size,
            }
        }
    }

    pub fn set_extent(mut self, width : u32, height : u32, depth : u32) -> Self {
        self.texture_size = wgpu::Extent3d {
            width: width,
            height: height,
            depth_or_array_layers: depth,
        };

        self
    }

    pub fn set_dimensions(mut self, dims : i32) -> Self {
        match dims {
            1 => self.texture_dimensions = wgpu::TextureDimension::D1,
            2 => self.texture_dimensions = wgpu::TextureDimension::D2,
            3 => self.texture_dimensions = wgpu::TextureDimension::D3,
            _ => self.texture_dimensions = wgpu::TextureDimension::D2,
        }            

        self
    }

    pub fn set_format(mut self, format : wgpu::TextureFormat) -> Self {
        self.texture_format = format;
        self
    }

    pub fn set_usage(mut self, usage : wgpu::TextureUsages) -> Self {
        self.texture_usage = usage;
        self
    }

    pub fn build(
        &self,
        device : &wgpu::Device,
        queue : &wgpu::Queue,
    ) -> wgpu::TextureView {
        let diffuse_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                size: self.texture_size,
                mip_level_count: 1, 
                sample_count: 1,
                dimension: self.texture_dimensions,
                format: self.texture_format,
                usage: self.texture_usage,
                label: None,
            }
        );

        if self.texture_type != TextureType::TextureDepth {
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &diffuse_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &self.pixel_data,
                self.image_layout,
                self.texture_size,
            );
        }

        diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }
}


pub fn create_sampler(device : &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        compare: Some(wgpu::CompareFunction::LessEqual), // 5.
        lod_min_clamp: 0.0,
        lod_max_clamp: 100.0,
        ..Default::default()
    })
}