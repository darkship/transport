use std::io;
mod endpoint;
mod file;

pub struct Transport<'a,F: 'a + endpoint::Endpoint, T: 'a + endpoint::Endpoint> {
    from: &'a mut F,
    to: &'a mut T,
}

impl <'a,F: 'a + endpoint::Endpoint, T: 'a + endpoint::Endpoint> Transport<'a,F, T> {
    pub fn forward (&self) -> Result<(), std::io::Error>{
        return copy(&'a self.from, &'a mut self.to)
    }
    pub fn backward (&self) -> Result<(), std::io::Error>{
       return copy(&self.to, &self.from)
    }

    
}

fn copy<'b>( from:&'b mut endpoint::Endpoint, to: &'b mut endpoint::Endpoint)-> Result<(), std::io::Error>{
        let mut buff = [0; 1024];
        let mut reading = true;
        while reading {
            match from.read(&mut buff) {
                Err(error) => {
                    return Err(error)
                },
                Ok(a) => {
                    if a == 0{
                        reading = false
                    } else {
                        match to.write(&mut buff){
                            Err(error) => {
                                return Err(error)
                            },
                            Ok(_) =>{}
                        }
                    }
                },
            }
        }
        Ok(())
    }

/*pub fn new<'a,F: 'a + endpoint::Endpoint, T: endpoint::Endpoint>(from: F, to: T) -> Transport<'a, F, T>{
    Transport<'a, 'a F, 'a T>{
        from: &'a mut from,
        to: &'a mut to
    }
}*/


#[cfg(test)]
mod tests {
    use super::*;
    struct TestEndPoint{

    }
    impl endpoint::Endpoint for TestEndPoint{
        #[allow(unused)]
        fn read(&mut self, buff: &mut [u8]) -> io::Result<usize>{
            Ok(0)
        }
        #[allow(unused)]
        fn write(&mut self, buff: &[u8]) -> io::Result<usize>{
            Ok(0)
        }
    }
    #[test]
    fn it_works() {
        let start = TestEndPoint{};
        let from = TestEndPoint{};
        let t = new(start, from);
        assert!(t.backward().is_ok());
        assert!(t.forward().is_ok());
        
    }

    /* #[test]
    fn it_works_with_file() {
        let mut file = std::fs::File::create("./testdata/source")?;
        file.write_all(b"Hello, world!")?;
        drop(file)
        //
        let start = 
        let from = TestEndPoint{};
        let t = new(start, from);
        assert!(t.backward().is_ok());
        assert!(t.forward().is_ok());
        
    }*/
}
