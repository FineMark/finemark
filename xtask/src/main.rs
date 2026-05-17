use std::process::{Command, exit};
use std::thread::sleep;
use std::time::Duration;

/// Crates published in dependency order.
const CRATES: &[&str] = &["finemark-ast", "finemark-parser"];

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("publish") => publish(false),
        Some("publish-dry") => publish(true),
        _ => {
            eprintln!("Usage: cargo xtask <command>");
            eprintln!();
            eprintln!("Commands:");
            eprintln!("  publish      Publish all crates to crates.io (in dependency order)");
            eprintln!("  publish-dry  Dry-run publish (no network writes)");
            exit(1);
        }
    }
}

fn publish(dry_run: bool) {
    println!(
        "Publishing FineMark crates{}...\n",
        if dry_run { " (dry run)" } else { "" }
    );

    for (i, crate_name) in CRATES.iter().enumerate() {
        println!("[{}/{}] Publishing {}...", i + 1, CRATES.len(), crate_name);

        let mut cmd = Command::new("cargo");
        cmd.arg("publish").arg("-p").arg(crate_name);

        if dry_run {
            cmd.arg("--dry-run");
        }

        let status = cmd.status().unwrap_or_else(|e| {
            eprintln!("Failed to run cargo publish: {e}");
            exit(1);
        });

        if !status.success() {
            eprintln!("Failed to publish {crate_name}");
            exit(1);
        }

        println!("{crate_name} published successfully.\n");

        // Wait for crates.io index to propagate before publishing the next
        // dependent crate. Skip the wait after the last crate and in dry runs.
        if !dry_run && i < CRATES.len() - 1 {
            println!("Waiting 10 s for crates.io index sync...\n");
            sleep(Duration::from_secs(10));
        }
    }

    println!("All crates published successfully!");
}
