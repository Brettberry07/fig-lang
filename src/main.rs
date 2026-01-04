mod evalulator;
mod enviorment;
mod helper;
mod lexer;
mod parser;
mod token;
mod types;

use std::{env, fs, io, path::{Path, PathBuf}};
use lexer::Lexer;
use parser::Parser;
use evalulator::eval_program;

fn main() {
    // CLI: fig [run [path]] | new <name>
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None => {
            // cargo run (no args) -> run ./src/main.fg or ./main.fg
            run_entry_default().unwrap_or_else(|e| {
                eprintln!("error: {e}");
                std::process::exit(1);
            });
        }
        Some("run") => {
            // fig run [path] (path can be a directory or a file)
            let target = args.next().unwrap_or_else(|| ".".to_string());
            run_target(&target).unwrap_or_else(|e| {
                eprintln!("error: {e}");
                std::process::exit(1);
            });
        }
        Some("new") => {
            // fig new <name>
            let name = args.next().unwrap_or_else(|| {
                eprintln!("usage: fig new <project-name>");
                std::process::exit(2);
            });
            scaffold_project(&name).unwrap_or_else(|e| {
                eprintln!("error: {e}");
                std::process::exit(1);
            });
            println!("Created {name}/ with src/main.fg");
        }
        Some(cmd) => {
            eprintln!("unknown command: {cmd}\nusage:\n  fig run [path]\n  fig new <name>");
            std::process::exit(2);
        }
    }
}

fn run_entry_default() -> io::Result<()> {
    // Prefer ./src/main.fg, fallback to ./main.fg
    let entry = find_entry(Path::new("."))?
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no src/main.fg or main.fg found"))?;
    run_file(&entry)
}

fn run_target(target: &str) -> io::Result<()> {
    let p = Path::new(target);
    let entry = if p.is_dir() {
        find_entry(p)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no src/main.fg or main.fg found in directory"))?
    } else {
        p.to_path_buf()
    };
    run_file(&entry)
}

fn find_entry(root: &Path) -> io::Result<Option<PathBuf>> {
    let src_main = root.join("src").join("main.fg");
    if src_main.exists() {
        return Ok(Some(src_main));
    }
    let root_main = root.join("main.fg");
    if root_main.exists() {
        return Ok(Some(root_main));
    }
    Ok(None)
}

fn run_file(path: &Path) -> io::Result<()> {
    let source = fs::read_to_string(path)?;
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse();
    // Run program; printing is handled by the 'print' builtin
    let _ = eval_program(&program);
    Ok(())
}

fn scaffold_project(name: &str) -> io::Result<()> {
    let root = Path::new(name);
    let src = root.join("src");
    fs::create_dir_all(&src)?;
    let main_fg = src.join("main.fg");
    if !main_fg.exists() {
        fs::write(&main_fg, DEFAULT_MAIN)?;
    }
    Ok(())
}

const DEFAULT_MAIN: &str = r#"# Fig starter
var x = 0;

for i in range(3) {
    print(i);
}

fn add(a, b) {
    return a + b;
}

print(add(3, 4));
"#;
