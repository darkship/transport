
use super::endpoint;
use std::io;
use std::io::Read;
use std::io::Write;
use std::fs;

pub struct File<'a>{
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::fs;
    #[test]
    fn reading() {
        let msg = b"Hello, world! \nssss\nsssss\nppppp";
        let filePath = "./data.test";
        let v = ||-> io::Result<()>{
            println!("was print");
            let mut f = fs::File::create(filePath)?;
            f.write_all(msg);
            

            let file = File{
            file: &fs::File::open("data.test")?
            };
            let mut buff = [0; 10];
            let mut reading = true;
            // let mut res = [];
            // loop {
            //     let n = file.read(buff)?;
            //     res = [res[..], ..buff];
            //     if n==0{
            //         break;
            //     }
            // }

            //assert_eq!(msg, buff);

            Ok(())
        };
        let _= v();
    }
}
