#![forbid(unsafe_code)]

use anim_editor::{blank_clip, check_clip};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!(
            "usage:\n  anim-editor write <file> --id N --name NAME --frames N --speed N [--loop]\n  anim-editor check <file>\n  anim-editor --check <file>"
        );
        return Ok(());
    }
    match args[0].as_str() {
        "check" | "--check" => {
            let path = args.get(1).ok_or("usage: anim-editor check <file>")?;
            let bytes = fs::read(path).map_err(|error| error.to_string())?;
            check_clip(&bytes).map_err(|error| error.to_string())
        }
        "write" => write(&args[1..]),
        _ => Err("unknown command; try anim-editor --help".into()),
    }
}

fn write(args: &[String]) -> Result<(), String> {
    let path = PathBuf::from(args.first().ok_or("missing output path")?);
    let id = flag(args, "--id")?.parse().map_err(|_| "bad --id")?;
    let name = flag(args, "--name")?;
    let frames = flag(args, "--frames")?
        .parse()
        .map_err(|_| "bad --frames")?;
    let speed = flag(args, "--speed")?.parse().map_err(|_| "bad --speed")?;
    let looping = args.iter().any(|arg| arg == "--loop");
    let bytes = blank_clip(id, &name, frames, speed, looping).map_err(|error| error.to_string())?;
    fs::write(path, bytes).map_err(|error| error.to_string())
}

fn flag(args: &[String], name: &str) -> Result<String, String> {
    args.iter()
        .position(|arg| arg == name)
        .and_then(|index| args.get(index + 1))
        .cloned()
        .ok_or_else(|| format!("missing {name}"))
}
