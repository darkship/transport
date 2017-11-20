use std::io;

pub trait Endpoint{
    fn read(&self, buff: &mut [u8]) -> io::Result<usize>;
    fn write(&self, buff: &[u8]) -> io::Result<usize>;
}
