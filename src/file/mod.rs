
use endpoint;
use std::io;
use std::io::Read;
use std::io::Write;
use std::fs;

pub struct File<'a> {
    file: &'a fs::File,
}
impl<'a> endpoint::Endpoint for File<'a> {
    fn read(&mut self, buff: &mut [u8]) -> io::Result<usize> {
        self.file.read(buff)
    }
    fn write(&mut self, buff: &[u8]) -> io::Result<usize> {
        self.file.write(buff)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::fs;
    use endpoint::Endpoint;

    #[test]
    fn reading() {
        let msg = b"Hello, world! \nssss\nsssss\nppppp";
        let filePath = "./data.test";
        let v = || -> io::Result<()> {
            let mut f = fs::File::create(filePath)?;
            f.write_all(msg);

            let mut f = File { file: &fs::File::open(filePath)? };
            let mut buff: [u8; 10] = [0; 10];
            let mut i: usize = 0;
            loop {
                let n = f.read(&mut buff)?;
                if n == 0 {
                    break;
                }
                assert_eq!(msg[i..i + n], buff[0..n]);
                i += n
            }
            assert_eq!(msg.len(), i);
            fs::remove_file(filePath)?;
            Ok(())
        };
        assert!(v().is_ok());

    }
}
