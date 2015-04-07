use std::mem;

use gfx::{
    BufferUsage,
    Factory,
    RawBufferHandle,
    Resources,
};

pub fn grow_buffer<R: Resources, F: Factory<R>, T>(
    factory: &mut F,
    buffer: &RawBufferHandle<R>,
    required_size: usize,
) -> RawBufferHandle<R> {
    let required_size_bytes = required_size * mem::size_of::<T>();
    let mut size = buffer.get_info().size;
    while size < required_size_bytes {
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
