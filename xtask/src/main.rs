use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: cargo xtask <command>");
        eprintln!("commands: serve");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "serve" => serve(),
        other => {
            eprintln!("unknown command: {}", other);
            std::process::exit(1);
        }
    }
}

fn serve() {
    let port = 8080;
    let site_dir = "portfolio";

    let mut miniserve: Child = Command::new("miniserve")
        .args([
            site_dir,
            "-p",
            &port.to_string(),
            "--index",
            "index.html",
            "--spa",
            "--color-scheme",
            "squirrel",
        ])
        .spawn()
        .expect("failed to start miniserve — is it installed?");

    sleep(Duration::from_millis(300));

    let url = format!("http://localhost:{}", port);
    println!("serving at {}", url);

    Command::new("open")
        .args(["-a", "Safari", &url])
        .spawn()
        .ok();

    miniserve.wait().ok();
}
