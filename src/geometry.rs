pub struct Geometry {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub layout: wgpu::VertexBufferLayout,
}

impl Geometry {
    pub fn new(device: &wgpu::Device, vertices: &[f32], indices: &[u32]) -> Self {
        
    }
}


