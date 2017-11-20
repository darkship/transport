

struct File{

}
impl endpoint::Endpoint for File{
    #[allow(unused)]
    fn read(&self, buff: &mut [u8]) -> io::Result<usize>{
        Ok(0)
    }
    #[allow(unused)]
    fn write(&self, buff: &[u8]) -> io::Result<usize>{
        Ok(0)
    }
}
