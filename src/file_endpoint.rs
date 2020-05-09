use endpoint::Endpoint;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

pub struct FileEndpoint<'a> {
    file: &'a fs::File,
}

impl<'a> Endpoint for FileEndpoint<'a> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.file.read(buffer)
    }
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.file.write(buffer)
    }
}

pub fn new<'a>(file: &'a fs::File) -> FileEndpoint<'a> {
    FileEndpoint { file }
}
