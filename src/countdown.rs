use std::io::{self, Write};
use std::time::{Duration, Instant};

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H"); // ANSI escape code to clear the terminal and move the cursor to the top-left.
    io::stdout().flush().unwrap();
}

pub fn format_time(seconds: u64) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

fn countdown_timer(duration: Duration) {
  let start_time = Instant::now();
  let end_time = start_time + duration;

  loop {
      let now = Instant::now();

      // Break the loop if we've reached or passed the end time.
      if now >= end_time {
          break;
      }

      // Calculate remaining time
      let remaining = end_time - now;
      let remaining_secs = remaining.as_secs();

      // Print the time inline
      print!("\r{}", format_time(remaining_secs));
      io::stdout().flush().unwrap();

      // Sleep for a short duration to prevent excessive CPU usage
      std::thread::sleep(Duration::from_millis(50));
  }

  print!("\r00:00\n"); // Ensure the final time is displayed as 00:00
}

pub fn start(minutes: u64) {
    clear_terminal();
    let countdown_duration = Duration::from_secs(minutes * 60); // 25 minutes
    countdown_timer(countdown_duration);
}
