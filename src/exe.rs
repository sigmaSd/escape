use std::io::Read;
use std::process::{Child, Command, Stdio};

pub fn execute(cmd: &str) -> String {
    let mut mem = None;

    for c in cmd.split('|') {
        mem = Some(run(mem, c));
    }

    let mut s = String::new();
    mem.unwrap().stdout.unwrap().read_to_string(&mut s).unwrap();
    s
}
fn run(mem: Option<Child>, c: &str) -> Child {
    let mut c = c.trim().split(' ');

    let mem = if mem.is_some() {
        mem.unwrap().stdout
    } else {
        None
    };

    if let Some(mem) = mem {
        Command::new(c.nth(0).unwrap())
            .args(&c.collect::<Vec<&str>>())
            .stdin(mem)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
    } else {
        Command::new(c.nth(0).unwrap())
            .args(&c.collect::<Vec<&str>>())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
    }
}
