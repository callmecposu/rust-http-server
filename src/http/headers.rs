use std::{ collections::HashMap, io::{ BufRead, BufReader }, net::TcpStream };

pub fn parse_headers(
    headers: &mut HashMap<String, String>,
    buf_reader: &mut BufReader<&mut &TcpStream>
) {
    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line);
        if line.trim().is_empty() {
            break;
        }
        let kv = line
            .split(":")
            .map(|x| x.trim().to_string())
            .collect::<Vec<String>>();
        headers.insert(kv[0].clone(), kv[1].clone());
    }
}
