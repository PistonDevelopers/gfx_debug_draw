use std::mem;

use gfx::{
    handle,
    BufferUsage,
    Factory,
    Resources,
    BufferRole,
};

pub fn grow_buffer<R: Resources, F: Factory<R>, T>(
    factory: &mut F,
    buffer: &handle::Buffer<R, T>,
    buffer_role: BufferRole,
    required_size: usize,
) -> handle::Buffer<R, T> {
    let mut size = buffer.len();
    while size < required_size {
        size *= 2;
    }
    factory.create_buffer_dynamic(size, buffer_role)
}

// pub fn grow_buffer<R: Resources, F: Factory<R>, T>(
//     factory: &mut F,
//     buffer: &handle::RawBuffer<R>,
//     required_size: usize,
// ) -> handle::RawBuffer<R> {
//     let required_size_bytes = required_size * mem::size_of::<T>();
//     let mut size = buffer.get_info().size;
//     while size < required_size_bytes {
//         size *= 2;
//     }
//     factory.create_buffer_raw(size, BufferUsage::Dynamic)
// }

pub static MAT4_ID: [[f32; 4]; 4] =
[
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];
