use anyhow::Result;
use image::DynamicImage;

use crate::gpu::{ComputePipeline, GpuContext};

const HEIGHT_SHADER: &str = include_str!("shaders/height.wgsl");
const NORMAL_SHADER: &str = include_str!("shaders/normal.wgsl");
const METALLIC_SHADER: &str = include_str!("shaders/metallic.wgsl");

pub struct PbrMaps {
    pub height: Vec<f32>,
    pub normal: Vec<u8>,
    pub metallic: Vec<u8>,
}

pub struct Pipeline {
    gpu: GpuContext,
    height_pipeline: ComputePipeline,
    normal_pipeline: ComputePipeline,
    metallic_pipeline: ComputePipeline,
}

impl Pipeline {
    pub async fn new() -> Result<Self> {
        let gpu = GpuContext::new().await?;

        let height_pipeline = gpu.create_compute_pipeline(
            HEIGHT_SHADER,
            "main",
            wgpu::TextureFormat::Rgba8Unorm, // input
            wgpu::TextureFormat::R32Float,   // output
        )?;
        let normal_pipeline = gpu.create_compute_pipeline(
            NORMAL_SHADER,
            "main",
            wgpu::TextureFormat::R32Float,   // input (height)
            wgpu::TextureFormat::Rgba8Unorm, // output
        )?;
        let metallic_pipeline = gpu.create_compute_pipeline(
            METALLIC_SHADER,
            "main",
            wgpu::TextureFormat::Rgba8Unorm, // input
            wgpu::TextureFormat::Rgba8Unorm, // output (R channel only used)
        )?;

        Ok(Self {
            gpu,
            height_pipeline,
            normal_pipeline,
            metallic_pipeline,
        })
    }

    pub async fn process(&self, image: &DynamicImage) -> Result<PbrMaps> {
        let width = image.width();
        let height = image.height();

        let diffuse_texture = self.gpu.create_texture_from_image(image);
        let diffuse_view = diffuse_texture.create_view(&Default::default());

        // 1. Generate height map
        let height_texture = self
            .gpu
            .create_output_texture(width, height, wgpu::TextureFormat::R32Float);
        let height_view = height_texture.create_view(&Default::default());

        let height_bind_group = self.gpu.create_bind_group(
            &self.height_pipeline.bind_group_layout,
            &diffuse_view,
            &height_view,
        );

        let workgroups_x = width.div_ceil(8);
        let workgroups_y = height.div_ceil(8);

        self.gpu.dispatch_compute(
            &self.height_pipeline.pipeline,
            &height_bind_group,
            workgroups_x,
            workgroups_y,
        );

        // 2. Generate normal map from height
        let normal_texture = self
            .gpu
            .create_output_texture(width, height, wgpu::TextureFormat::Rgba8Unorm);
        let normal_view = normal_texture.create_view(&Default::default());

        let normal_bind_group = self.gpu.create_bind_group(
            &self.normal_pipeline.bind_group_layout,
            &height_view,
            &normal_view,
        );

        self.gpu.dispatch_compute(
            &self.normal_pipeline.pipeline,
            &normal_bind_group,
            workgroups_x,
            workgroups_y,
        );

        // 3. Generate metallic map from diffuse (Rgba8Unorm: only R channel used)
        let metallic_texture = self
            .gpu
            .create_output_texture(width, height, wgpu::TextureFormat::Rgba8Unorm);
        let metallic_view = metallic_texture.create_view(&Default::default());

        let metallic_bind_group = self.gpu.create_bind_group(
            &self.metallic_pipeline.bind_group_layout,
            &diffuse_view,
            &metallic_view,
        );

        self.gpu.dispatch_compute(
            &self.metallic_pipeline.pipeline,
            &metallic_bind_group,
            workgroups_x,
            workgroups_y,
        );

        // Read back results - we need to wait for GPU to finish before readback
        self.gpu.device.poll(wgpu::Maintain::Wait);

        let height_data = self.gpu.read_texture(&height_texture).await?;
        let normal_data = self.gpu.read_texture(&normal_texture).await?;
        let metallic_data = self.gpu.read_texture(&metallic_texture).await?;

        let height_f32: Vec<f32> = height_data
            .chunks_exact(4)
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();

        // Metallic was written as RGBA (only R used); extract R channel
        let metallic_r: Vec<u8> = metallic_data.chunks_exact(4).map(|c| c[0]).collect();

        Ok(PbrMaps {
            height: height_f32,
            normal: normal_data,
            metallic: metallic_r,
        })
    }
}
