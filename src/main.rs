// https://doc.rust-lang.org/std/process/index.html
use std::process::Command;
use std::env;

struct Fixes {
    
}

fn sources(dir: &str) -> impl Iterator<Item = String> {
    if true {
        std::iter::empty()
    } else {
        std::iter::once(|| String::new("sdf"))
    }
}

fn compile(file: &str) {
    let mut child = Command::new("cc")
        .arg(file)
        .arg("-o")
        .arg("test/a.out")
        .spawn()
        .expect("failed to spawn child");
    child.wait().expect("command wasn't running");
}

fn main() {
    for dir in env::args() {
        let sources = sources(&dir);
        for source in sources {
            compile(&source);
        }
    }
}
