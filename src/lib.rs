use std::io;

pub trait Endpoint{
    fn read(&self, buf: &mut [u8]) -> io::Result<usize>;
    fn write(&self);
}

pub struct Transport<F: Endpoint, T: Endpoint> {
    from: F,
    to: T,
}

impl <F: Endpoint, T: Endpoint> Transport<F, T> {
    pub fn forward (&self) -> Result<(), std::io::Error>{
        let mut buff = [0; 1024];
        self.from.read(&mut buff[..])?;
        self.to.write();
        Ok(())
    }
    pub fn backward (&self) -> Result<bool, std::io::Error>{
        let mut buff = [0; 1024];
        self.to.read(&mut buff[..])?;
        self.from.write();
        Ok(true)
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
        fn read(&self, buf: &mut [u8]) -> io::Result<usize>{
            Ok(0)
        }
        fn write(&self){}
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
