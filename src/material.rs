use std::sync::Arc;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Material {
    pub pipeline: Arc<wgpu::RenderPipeline>,
    pub bind_group: wgpu::BindGroup,
}






