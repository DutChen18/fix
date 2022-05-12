// https://doc.rust-lang.org/std/process/index.html
pub mod compiler;

use std::env;
use std::fs;
use std::path;

use compiler::*;

//TODO make function that takes a file and prepends a header, using its path

fn prepend(s1: String, mut s2: String) -> String {
    s2.push_str("\n");
    s2.push_str(&s1);
    s2
}

fn prepend_header(s1: String, p: &path::Path) -> String {
    let mut include: String = "#include \"".to_owned();
    let path_string = p.to_str().expect("Path can't be converted to string");
    include.push_str(path_string);
    include.push_str("\"");
    include.push_str("\n");
    include.push_str(&s1);
    include
}

fn sources(dir: &path::Path, extension: &str) -> impl Iterator<Item = path::PathBuf> {
    let paths = fs::read_dir(dir).expect("invalid directory");
    let owned_extension = extension.to_owned();
    paths.filter_map(move |x| {
        let entry = x.expect("failed to get directory entry");
        let is_file = entry
            .file_type()
            .expect("could not determine file type")
            .is_file();
        let has_extension = entry.path().extension() == Some(owned_extension.as_ref());
        if is_file && has_extension {
            return Some(
                entry
                    .path()
                    .to_owned(),
            );
        }
        None
    })
}

fn main() {
    let dir = env::args().nth(1).expect("not enough arguments");
    let dir = path::Path::new(&dir);
    let sources = sources(dir, "c");
    for source in sources {
        println!("{} {:?}", source.to_str().unwrap(), Clang::compile(&CompileInput { filename: source.to_owned() }));
    }
}
