use std::{io, thread, time::Duration};
use std::io::Write;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

fn main() {
    println!("⏳ Welcome to Basic Timer Tool");
    println!("Enter the time duration (format: Hours Minutes Seconds):");
    
    let duration = match get_duration() {
        Some(dur) => dur,
        None => {
            println!("❌ Invalid input. Please enter a valid time duration. (e.g., 1 30 40 for 1 hour, 30 minutes, and 40 seconds)");
            return;
        }
    };

    println!("✅ Timer set for: {} hours, {} minutes, {} seconds", 
             duration.0, duration.1, duration.2);

    // Shared pause flag
    let paused = Arc::new(AtomicBool::new(false));
    let pause_flag = Arc::clone(&paused);

    // Spawn input thread
    thread::spawn(move || {
        let stdin = io::stdin();
        loop {
            let mut buf = String::new();
            if stdin.read_line(&mut buf).is_ok() {
                if buf.trim() == "p" {
                    let currently_paused = pause_flag.load(Ordering::SeqCst);
                    pause_flag.store(!currently_paused, Ordering::SeqCst);
                    if currently_paused {
                        println!("\n▶️  Resumed!");
                    } else {
                        println!("\n⏸️  Paused! Press 'p' again to resume.");
                    }
                }
            }
        }
    });

    start_timer(duration.0, duration.1, duration.2, paused);

    println!("🎉 Time's up!");
}

fn get_duration() -> Option<(u64, u64, u64)> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 3 {
        return None;
    }

    let hours = parts[0].parse::<u64>().ok()?;
    let minutes = parts[1].parse::<u64>().ok()?;
    let seconds = parts[2].parse::<u64>().ok()?;

    Some((hours, minutes, seconds))
}

fn start_timer(hours: u64, minutes: u64, seconds: u64, paused: Arc<AtomicBool>) {
    let total_seconds = hours * 3600 + minutes * 60 + seconds;
    for remaining in (0..total_seconds).rev() {
        // Pause logic
        while paused.load(std::sync::atomic::Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(200));
        }

        let hrs = remaining / 3600;
        let mins = (remaining % 3600) / 60;
        let secs = remaining % 60;

        print!("\r⏰ Time remaining: {:02}:{:02}:{:02}  (press 'p' + Enter to pause/resume)", hrs, mins, secs);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!();
}