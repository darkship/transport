use std::io;

pub trait Endpoint {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize>;
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize>;
}
