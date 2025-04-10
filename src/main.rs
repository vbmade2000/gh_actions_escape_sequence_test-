use std::{thread, time::Duration};

fn main() {
    for i in 0..=100 {
        print!("\r\033[KProgress: {}%", i);
        // Flush stdout to ensure the print is shown immediately
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    // Print a newline at the end
    println!();
}