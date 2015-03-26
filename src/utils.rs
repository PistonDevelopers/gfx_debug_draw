use std::mem;

use gfx::{
    RawBufferHandle,
    BufferUsage,
};

use gfx::traits::*;

pub fn grow_buffer<D: Device, F: Factory<D::Resources>, T>(
    factory: &mut F,
    buffer: &RawBufferHandle<D::Resources>,
    required_size: usize,
) -> RawBufferHandle<D::Resources> {
    let mut size = buffer.get_info().size / mem::size_of::<T>();
    while size < required_size {
        size *= 2;
    }
    factory.create_buffer_raw(size, BufferUsage::Dynamic)
}

pub static MAT4_ID: [[f32; 4]; 4] =
[
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];
