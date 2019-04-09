use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{self, Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};

struct Con {
	listener: TcpListener,
	stream: Arc<Mutex<String>>,
}

impl Con {
	fn connect() -> io::Result<Self> {
		let listener = TcpListener::bind("127.0.0.1:8080")?;
		Ok(Self
		{
			listener,
			stream: Arc::new(Mutex::new(String::new())),
		})
	}
	fn start(&mut self) -> io::Result<()> {
		for stream in self.listener.incoming() {
			  let d = self.stream.clone();
			  thread::spawn(move ||{  
			  	let mut s = stream.unwrap();
   		        s.read_to_string(&mut d.lock().unwrap()).unwrap();
   	        	Self::get_cmd(d.lock().unwrap().to_string());
   	        	});
			//Self::stop_con(s);
		}
		/*for stream in self.listener.incoming() {
			let mut s = stream?;
	        s.read_to_string(&mut self.stream)?;
	        self.get_cmd();
	        s.shutdown(Shutdown::Both);
	        drop(s);
	    }*/
	    Ok(())
	}
	fn stop_con(mut stream: TcpStream) {
		let status = String::from("HTTP/1.1 200 OK\r\n\r\n");
		let mut response = status.as_bytes().to_vec();
		stream.write_all(&response).unwrap();
		stream.flush().unwrap();
	}
	fn get_cmd(d: String) {
		let http_idx = match d.find("HTTP") {
			Some(idx) => idx - 1,
			None => return,
		};
		let cmd = &d[5..http_idx];
		dbg!(&cmd);
	}   	
}

fn main() -> io::Result<()> {
	let mut con = Con::connect()?;
	con.start();
	Ok(())
}