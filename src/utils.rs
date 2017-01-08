use gfx::{
    buffer,
    handle,
    Bind,
    Factory,
    Resources,
};

pub fn grow_buffer<R: Resources, F: Factory<R>, T>(
    factory: &mut F,
    buffer: &handle::Buffer<R, T>,
    buffer_role: buffer::Role,
    required_size: usize,
) -> handle::Buffer<R, T> {
    let mut size = buffer.len();
    while size < required_size {
        size *= 2;
    }
    factory.create_buffer_dynamic(size, buffer_role, Bind::empty())
        .expect("Could not create buffer")
}
