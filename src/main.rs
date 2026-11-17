use std::env;
use std::io;

pub mod count;
pub mod file;

fn fmt_val(n: i64) -> String {
    if n == -1 {
        "?".to_string()
    } else {
        n.to_string()
    }
}

fn fmt_ext(raw: &str) -> String {
    let label = if raw.is_empty() || raw == "(none)" {
        "(none)".to_string()
    } else if raw.starts_with('.') || raw == "LICENSE" {
        raw.to_string()
    } else {
        format!(".{raw}")
    };

    if label.chars().count() > 10 {
        let trimmed: String = label.chars().take(7).collect();
        format!("{trimmed}...")
    } else {
        label
    }
}

fn print_help() {
    println!("dirlines - A fast recursive code line counter");
    println!("\nUsage:");
    println!("  dirlines [PATH]         Count lines in the given directory (default: current directory)");
    println!("  dirlines --help, -h     Print this help message");
    println!("  dirlines --version, -V  Print version information");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            "--version" | "-V" => {
                println!("dirlines 0.1.0");
                println!("https://github.com/cvrspark/dirlines.git");
                return Ok(());
            }
            _ => {}
        }
    }

    let target_dir = args.get(1).map(|s| s.as_str()).unwrap_or(".");

    let mut w = file::Workspace::new(target_dir);
    w.load_files()?;
    let map = count::count(&w);

    println!("{:<10} | {:>8} | {:>8} | {:>8}", "Ext", "Code", "Comment", "Void");
    println!("{}", "-".repeat(43));

    let mut total_code: i64 = 0;
    let mut total_comment: i64 = 0;
    let mut total_void: i64 = 0;

    for (k, v) in &map {
        total_code += if v.code != -1 { v.code } else { 0 };
        total_comment += if v.comment != -1 { v.comment } else { 0 };
        total_void += if v.void != -1 { v.void } else { 0 };

        println!(
            "{:<10} | {:>8} | {:>8} | {:>8}",
            fmt_ext(k),
            fmt_val(v.code),
            fmt_val(v.comment),
            fmt_val(v.void),
        );
    }
    
    println!("{}", "-".repeat(43));
    
    println!(
        "{:<10} | {:>8} | {:>8} | {:>8}",
        "Total",
        total_code,
        total_comment,
        total_void,
    );

    Ok(())
}