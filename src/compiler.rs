use std::process::Command;
use std::io::Read;
use std::process::Stdio;
use std::path;

pub struct CompileInput {
    pub filename: path::PathBuf,
}

#[derive(Debug)]
pub enum CompileResult {
    ImplicitDeclaration {
        function: String
    }
}

pub trait Compiler {
    fn compile(info: &CompileInput) -> Vec<CompileResult>;
}

pub struct Clang;

impl Clang {
    fn parse_line(line: &str) -> Option<CompileResult> {
        let parts = line.split(": ").collect::<Vec<_>>();
        let location = parts.get(0)?.split(':').collect::<Vec<_>>();
        let severity = *parts.get(1)?;
        let message = *parts.get(2)?;

        if severity == "warning" && message.starts_with("implicit declaration of function") {
            let parts = message.split(" ").collect::<Vec<_>>();
            let part = parts.get(4)?;
            return Some(CompileResult::ImplicitDeclaration {
                function: part[1..part.len() - 1].to_owned()
            });
        }

        return None;
    }
}

impl Compiler for Clang {
    fn compile(info: &CompileInput) -> Vec<CompileResult> {
        let mut child = Command::new("cc")
            .arg(&info.filename)
            .arg("-c")
            .arg("-fsyntax-only")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to spawn child");

        let stdout = child.stdout.as_mut().expect("failed to get stdout");
        let stderr = child.stderr.as_mut().expect("failed to get stderr");

        let mut string = String::new();
        stderr.read_to_string(&mut string).expect("read failed");
        let mut results = Vec::new();

        for line in string.split('\n') {
            if let Some(result) = Clang::parse_line(line) {
                results.push(result);
            }
        }

        child.wait().expect("failed to wait on child");

        results
    }
}