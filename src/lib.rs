use std::io;
mod endpoint;
mod file;

pub struct Transport<'a, F: 'a + endpoint::Endpoint, T: 'a + endpoint::Endpoint> {
    from: &'a mut F,
    to: &'a mut T,
}

impl<'a, F: 'a + endpoint::Endpoint, T: 'a + endpoint::Endpoint> Transport<'a, F, T> {
    pub fn forward(&mut self) -> Result<(), io::Error> {
        return copy(self.from, self.to);
    }
    pub fn backward(&mut self) -> Result<(), io::Error> {
        return copy(self.to, self.from);
    }
}

fn copy<'a>(
    from: &'a mut endpoint::Endpoint,
    to: &'a mut endpoint::Endpoint,
) -> Result<(), io::Error> {
    let mut buff = [0; 1024];
    let mut reading = true;
    while reading {
        match from.read(&mut buff) {
            Err(error) => return Err(error),
            Ok(a) => {
                if a == 0 {
                    reading = false
                } else {
                    match to.write(&mut buff) {
                        Err(error) => return Err(error),
                        Ok(_) => {}
                    }
                }
            }
        }
    }
    Ok(())
}
pub fn new<'a, F: 'a + endpoint::Endpoint, T: 'a + endpoint::Endpoint>(
    from: &'a mut F,
    to: &'a mut T,
) -> Transport<'a, F, T> {
    return Transport { from: from, to: to };
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestEndPoint {}
    impl endpoint::Endpoint for TestEndPoint {
        #[allow(unused)]
        fn read(&mut self, buff: &mut [u8]) -> io::Result<usize> {
            Ok(0)
        }
        #[allow(unused)]
        fn write(&mut self, buff: &[u8]) -> io::Result<usize> {
            Ok(0)
        }
    }
    #[test]
    fn it_works() {
        let mut from = TestEndPoint {};
        let mut to = TestEndPoint {};
        let mut t = new(&mut from, &mut to);
        assert!(t.backward().is_ok());
        assert!(t.forward().is_ok());
    }
}
