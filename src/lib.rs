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
                    let mut result_content = String::new();
                    file.read_to_string(&mut result_content)?;
                    assert!(
                        result_content == msg,
                        "expected '{}' but got '{}'.",
                        msg,
                        result_content
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
        let file_path = "./from_file_to_file.test";

        let v = || -> io::Result<()> {
            let mut msg = String::with_capacity(9729);
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
                    let mut expected_content = String::new();
                    let mut from_file = &fs::File::open(file_path)?;
                    from_file.read_to_string(&mut expected_content)?;
                    assert!(
                        msg == expected_content,
                        "expected '{}' but got '{}'.",
                        expected_content,
                        msg
                    );
                }
            };

            Ok(())
        };

        assert!(v().is_ok());
    }

    #[test]
    fn from_file_to_file() {
        let from_file_path = "./from_file_to_file.test";
        let to_file_path = "./from_file_to_file_destination.test";

        let v = || -> io::Result<()> {
            let from_file = &fs::File::open(from_file_path)?;
            let mut from_file_endpoint = file_endpoint::new(from_file);

            let to_file = &fs::File::create(to_file_path)?;
            let mut to_file_endpoint = file_endpoint::new(to_file);

            let mut t = new(&mut from_file_endpoint, &mut to_file_endpoint);
            match t.forward() {
                Err(error) => {
                    println!("error: {}", error);
                    assert!(false);
                }
                Ok(_) => {
                    let mut expected_content = String::new();
                    let mut from_file = &fs::File::open(from_file_path)?;
                    from_file.read_to_string(&mut expected_content)?;

                    let mut file_result = &fs::File::open(to_file_path)?;
                    let mut result_content = String::new();
                    file_result.read_to_string(&mut result_content)?;

                    assert!(expected_content == result_content,);
                    fs::remove_file(to_file_path)?;
                }
            };

            Ok(())
        };

        assert!(v().is_ok());
    }
}
