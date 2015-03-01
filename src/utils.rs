use gfx::{
    BufferHandle,
    BufferUsage,
    Device,
    Graphics,
};

use gfx_device_gl::{
    GlDevice,
    GlResources,
};

pub fn grow_buffer<T>(
    graphics: &mut Graphics<GlDevice>,
    buffer: BufferHandle<GlResources, T>,
    required_size: usize,
) -> BufferHandle<GlResources, T> {
    let mut size = buffer.len();
    graphics.device.delete_buffer(buffer);
    while size < required_size {
        size *= 2;
    }
    graphics.device.create_buffer::<T>(size, BufferUsage::Dynamic)
}

pub static MAT4_ID: [[f32; 4]; 4] =
[
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];
