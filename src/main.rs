use std::io::{self, Write};
use std::env;
use std::process::Command;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break; // Ctrl-D (end of input)
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let args: Vec<String> = expand(&parts[1..]);
        match parts[0] {
            "exit" => break,
            "cd" => builtin_cd(&args),
            "ls" => builtin_ls(&args),
            "pwd" => println!("{}", env::current_dir().unwrap().display()),
            cmd => run_external(cmd, &args),
        }
    }
}

// builtin cd command
fn builtin_cd(args: &[String]) {
    let target = match args.first() {
        Some(path) => path.to_string(),
        None => match env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                eprintln!("cd: HOME directory is not set");
                return;
            }
        },
    };

    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: {}: {}", target, e);
    }
}

// builtin ls command
fn builtin_ls(args: &[String]) {
    let target = args.first().map(|s| s.as_str()).unwrap_or(".");

    let entries = match std::fs::read_dir(target) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("ls: {}: {}", target, e);
            return;
        }
    };

    let mut names: Vec<String> = Vec::new();
    for entry in entries {
        match entry {
            Ok(entry) => names.push(entry.file_name().to_string_lossy().to_string()),
            Err(e) => eprintln!("ls: {}", e),
        }
    }

    names.sort();
    for name in names {
        println!("{}", name);
    }
}

// run external commands
fn run_external(cmd: &str, args: &[String]) {
    match Command::new(cmd).args(args).status() {
        Ok(_) => {}
        Err(e) => eprintln!("{}: {}", cmd, e),
    }
}

// expand ~, . and *
fn expand(args: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for arg in args {
        if *arg == "~" || arg.starts_with("~/") {
            match env::var("HOME") {
                Ok(home) => {
                    if *arg == "~" {
                        result.push(home);
                    } else {
                        let rest = &arg[1..];
                        result.push(format!("{}{}", home, rest));
                    }
                }
                Err(_) => result.push(arg.to_string()),
            }
        } else {
            result.push(arg.to_string());
        }
    }

    result
}