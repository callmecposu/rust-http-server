use std::{ io::{ BufRead, BufReader, Read }, net::TcpStream };

#[derive(Debug, Default)]
pub struct HttpBody {
    pub raw: String
}

impl HttpBody {
    pub fn new() -> Self {
        HttpBody { ..Default::default() }
    }

    pub fn parse(&mut self, cont_length: i32, buf_reader: &mut BufReader<&mut &TcpStream>) {
        // read the body lines into the raw member
        for _ in 0..cont_length {
            let mut buf : [u8; 1] = [0];
            buf_reader.read_exact(&mut buf);
            self.raw.push(buf[0] as char);
        }
    }
}
