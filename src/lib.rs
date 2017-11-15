
pub mod transport {
    pub struct Transport {
        
    }

    impl Transport {
        pub fn forward (&self) -> Result<bool, &str>{
            Ok(true)
        }
        pub fn backward (&self) -> Result<bool, &str>{
            Ok(true)
        }
    }

    pub fn new() -> Transport{
        Transport{}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let t = transport::new();
        assert_eq!(t.forward(), Ok(true));
        assert_eq!(t.backward(), Ok(true));
    }
}
