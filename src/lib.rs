
pub trait Endpoint{
    fn read(&self);
    fn write(&self);
}

pub struct Transport<F: Endpoint, T: Endpoint> {
    from: F,
    to: T,
}

impl <F: Endpoint, T: Endpoint> Transport<F, T> {
    pub fn forward (&self) -> Result<bool, &str>{
        self.from.read();
        self.to.write();
        Ok(true)
    }
    pub fn backward (&self) -> Result<bool, &str>{
        self.to.read();
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
        fn read(&self){}
        fn write(&self){}
    }
    #[test]
    fn it_works() {
        let start = TestEndPoint{};
        let from = TestEndPoint{};
        let t = new(start, from);
        assert_eq!(t.forward(), Ok(true));
        assert_eq!(t.backward(), Ok(true));
    }
}
