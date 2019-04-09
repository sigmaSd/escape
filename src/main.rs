use std::io::{self, Read};
use std::net::TcpListener;
use std::process::Command;

struct Con {
    listener: TcpListener,
    stream: String,
}

impl Con {
    fn connect() -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:40566")?;
        Ok(Self {
            listener,
            stream: String::new(),
        })
    }
    fn start(&mut self) -> io::Result<()> {
        for stream in self.listener.incoming() {
            self.stream.clear();
            let mut s = stream?;
            s.read_to_string(&mut self.stream)?;
            if let Some(cmd) = self.get_cmd() {
                match Self::exec_cmd(cmd) {
                    Ok(_) => (),
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
    fn exec_cmd(mut cmd: std::str::Split<&str>) -> io::Result<()> {
        Command::new(cmd.nth(0).unwrap())
            .args(&cmd.collect::<Vec<&str>>())
            .spawn()?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut con = Con::connect()?;
    con.start()?;
    Ok(())
}
