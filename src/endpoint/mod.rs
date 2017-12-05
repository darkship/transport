use std::io;

pub trait Endpoint {
    fn read(&mut self, buff: &mut [u8]) -> io::Result<usize>;
    fn write(&mut self, buff: &[u8]) -> io::Result<usize>;
}
