use endpoint::*;
use std::fs;
use std::io::Read;
use std::io::Result;
use std::io::Write;

pub struct FileEndpoint<'a> {
    file: &'a fs::File,
}

impl<'a> EndpointFrom for FileEndpoint<'a> {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        self.file.read(buffer)
    }
}

impl<'a> EndpointTo for FileEndpoint<'a> {
    fn write(&mut self, buffer: &[u8]) -> Result<usize> {
        self.file.write(buffer)
    }
}

pub fn new<'a>(file: &'a fs::File) -> FileEndpoint<'a> {
    FileEndpoint { file }
}
