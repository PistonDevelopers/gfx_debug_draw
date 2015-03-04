use gfx::{
    BufferHandle,
    BufferUsage,
    Device,
    Graphics,
};

pub fn grow_buffer<D: Device, T>(
    graphics: &mut Graphics<D>,
    buffer: BufferHandle<D::Resources, T>,
    required_size: usize,
) -> BufferHandle<D::Resources, T> {
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
