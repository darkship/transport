use std::io;

pub trait Endpoint{
    fn read(&self, buff: &mut [u8]) -> io::Result<usize>;
    fn write(&self, buff: &[u8]) -> io::Result<usize>;
}

pub struct Transport<F: Endpoint, T: Endpoint> {
    from: F,
    to: T,
}

impl <F: Endpoint, T: Endpoint> Transport<F, T> {
    pub fn forward (&self) -> Result<(), std::io::Error>{
        return self.copy(&self.from, &self.to)
    }
    pub fn backward (&self) -> Result<(), std::io::Error>{
       return self.copy(&self.to, &self.from)
    }

    fn copy(&self, from:&Endpoint, to:&Endpoint)-> Result<(), std::io::Error>{
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
}

pub fn new<F: Endpoint, T: Endpoint>(from: F, to: T) -> Transport<F,T>{
    Transport{
        from: from,
        to: to
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    struct TestEndPoint{

    }
    impl Endpoint for TestEndPoint{
        #[allow(unused)]
        fn read(&self, buff: &mut [u8]) -> io::Result<usize>{
            Ok(0)
        }
        #[allow(unused)]
        fn write(&self, buff: &[u8]) -> io::Result<usize>{
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
}
