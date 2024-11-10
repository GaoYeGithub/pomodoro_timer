mod timer;
mod snapshot;
mod args;

use clap::Parser;
use args::Args;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use snapshot::{DirectorySnapshot, ChangeReport, ChangeType};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let dir = args.directory.unwrap_or_else(|| std::env::current_dir().unwrap());

    println!("{}", "\n🍅 Pomodoro Timer Starting 🍅".bright_red().bold());
    println!("Tracking directory: {}\n", dir.display());

    let pb_style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}s)")
        .unwrap()
        .progress_chars("=>-");

    println!("Taking initial snapshot...");
    let initial_snapshot = if let Some(snapshot_path) = args.initial_snapshot {
        println!("Loading initial snapshot from file: {}", snapshot_path.display());
        DirectorySnapshot::from_file(&snapshot_path)?
    } else {
        DirectorySnapshot::new(&dir)?
    };

    let pb = ProgressBar::new(args.work_duration * 60);
    pb.set_style(pb_style.clone());
    
    println!("\n{}", "🎯 Work Session Started".green().bold());
    timer::run_timer(args.work_duration, "Work session completed!", &pb).await;

    println!("\nTaking final snapshot...");
    let final_snapshot = DirectorySnapshot::new(&dir)?;

    println!("\n{}", "📊 Session Summary".blue().bold());
    println!("Changes during this session:");

    let changes = final_snapshot.compare(&initial_snapshot);
    let mut changes_found = false;

    for change in &changes {
        changes_found = true;
        match change.change_type {
            ChangeType::Modified => println!("  Modified: {}", change.path.yellow()),
            ChangeType::Added => println!("  Added: {}", change.path.green()),
            ChangeType::Deleted => println!("  Deleted: {}", change.path.red()),
        }
    }

    if !changes_found {
        println!("  No changes detected");
    }

    if changes_found {
        let change_report = ChangeReport {
            session_start: initial_snapshot.timestamp,
            session_end: final_snapshot.timestamp,
            changes,
        };

        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let report_path = format!("pomodoro_changes_{}.json", timestamp);
        std::fs::write(
            &report_path,
            serde_json::to_string_pretty(&change_report)?,
        )?;
        println!("\nChange report saved to: {}", report_path);
    }

    println!("\n{}", "⏰ Break Time!".cyan().bold());
    let pb = ProgressBar::new(args.break_duration * 60);
    pb.set_style(pb_style);
    timer::run_timer(args.break_duration, "Break completed!", &pb).await;

    Ok(())
}