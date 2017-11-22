
use super::endpoint;
use std::io;
use std::io::Read;
use std::io::Write;
use std::fs;

struct File<'a>{
    file : &'a fs::File
}
impl<'a> endpoint::Endpoint for File<'a>{
    fn read(&mut self, buff: &mut [u8]) -> io::Result<usize>{
       self.file.read(buff)
    }
    fn write(&mut self, buff: &[u8]) -> io::Result<usize>{
        self.file.write(buff)
    }
}
