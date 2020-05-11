use std::io;

pub trait EndpointFrom {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize>;
}

pub trait EndpointTo {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize>;
}
