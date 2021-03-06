use endpoint::*;
use std::io::Result;

pub struct StringEndpoint<'a> {
    string: &'a mut String,
    pos: usize,
}

impl<'a> EndpointFrom for StringEndpoint<'a> {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        let mut size: usize = 0;
        let string_as_bytes = self.string.as_bytes();
        loop {
            if self.pos >= string_as_bytes.len() || size == buffer.len() {
                break;
            }
            buffer[size] = string_as_bytes[self.pos];
            self.pos = self.pos + 1;
            size = size + 1;
        }
        Ok(size)
    }
}

impl<'a> EndpointTo for StringEndpoint<'a> {
    fn write(&mut self, buffer: &[u8]) -> Result<usize> {
        let new_str = String::from_utf8(buffer.to_vec()).unwrap();
        self.string.push_str(&new_str);
        Ok(buffer.len())
    }
}

pub fn new<'a>(str: &'a mut String) -> StringEndpoint<'a> {
    StringEndpoint {
        string: str,
        pos: 0,
    }
}
