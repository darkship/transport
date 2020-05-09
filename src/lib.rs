use std::io::Error;

pub mod endpoint;
pub mod file_endpoint;
pub mod string_endpoint;

pub struct Transport<'a, F: 'a + endpoint::Endpoint, T: 'a + endpoint::Endpoint> {
    from: &'a mut F,
    to: &'a mut T,
}

impl<'a, F: 'a + endpoint::Endpoint, T: 'a + endpoint::Endpoint> Transport<'a, F, T> {
    pub fn forward(&mut self) -> Result<(), Error> {
        return copy(self.from, self.to);
    }
}

fn copy<'a>(
    from: &'a mut dyn endpoint::Endpoint,
    to: &'a mut dyn endpoint::Endpoint,
) -> Result<(), Error> {
    let mut buffer = [0; 1024];
    let mut reading = true;
    while reading {
        match from.read(&mut buffer) {
            Err(error) => return Err(error),
            Ok(a) => {
                if a == 0 {
                    reading = false
                } else {
                    match to.write(&mut buffer[0..a]) {
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
    return Transport { from, to };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io;
    use std::io::prelude::*;

    struct TestEndPoint {}
    impl endpoint::Endpoint for TestEndPoint {
        #[allow(unused)]
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            Ok(0)
        }
        #[allow(unused)]
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            Ok(0)
        }
    }
    #[test]
    fn it_works() {
        let mut from = TestEndPoint {};
        let mut to = TestEndPoint {};
        let mut t = new(&mut from, &mut to);
        assert!(t.forward().is_ok());
    }

    #[test]
    fn from_string_to_file() {
        let file_path = "./from_string_to_file.test";
        let v = || -> io::Result<()> {
            let mut msg = String::from(
                "1- Hello, world! \nssss\nsssss\nppppp\n2-Hello, world! \nssss\nsssss\nppppp\n",
            );
            let mut from_string = string_endpoint::new(&mut msg);

            let file = &fs::File::create(file_path)?;
            let mut to_file = file_endpoint::new(file);

            let mut t = new(&mut from_string, &mut to_file);
            match t.forward() {
                Err(error) => {
                    println!("error: {}", error);
                    assert!(false);
                }
                Ok(_) => {
                    let mut file = &fs::File::open(file_path)?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;
                    assert!(
                        contents == msg,
                        "expected '{}' but got '{}'.",
                        msg,
                        contents
                    );
                    fs::remove_file(file_path)?;
                }
            };

            Ok(())
        };

        assert!(v().is_ok());
    }

    #[test]
    fn from_file_to_string() {
        let file_path = "./from_file_to_string.test";
        let v = || -> io::Result<()> {
            let expected_msg = String::from(
                "1- Hello, world! \nssss\nsssss\nppppp\n2-Hello, world! \nssss\nsssss\nppppp\n",
            );
            let mut msg = String::from("");

            let mut to_string = string_endpoint::new(&mut msg);

            let file = &fs::File::open(file_path)?;
            let mut from_file = file_endpoint::new(file);

            let mut t = new(&mut from_file, &mut to_string);
            match t.forward() {
                Err(error) => {
                    println!("error: {}", error);
                    assert!(false);
                }
                Ok(_) => {
                    assert!(
                        msg == expected_msg,
                        "expected '{}' but got '{}'.",
                        expected_msg,
                        msg
                    );
                }
            };

            Ok(())
        };

        assert!(v().is_ok());
    }
}
