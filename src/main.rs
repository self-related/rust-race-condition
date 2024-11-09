/**********
* This app spawns 2 threads that endlessly re-write the last line of the terminal
**********/

use std::io::{stdout, Write};
use std::time::Duration;

fn main() {
    // the scope joins all threads at the end and doesn't allow the program end before all work is done
    let _scope = std::thread::scope(|scope| {
        scope.spawn(|| loop {
            // sleeps for 200 milliseconds
            std::thread::sleep(Duration::from_millis(200));
            print!("\r=========>                              \x1b[100C"); // \x1b[100C moves cursor 100 chars to the right

            // print!("\x1b[2J"); // 2J clears screan, 1J clean up, 0J or J clean down

            _ = stdout().flush(); // print! doesn't write its buffer before flushing stdout
        });

        scope.spawn(|| loop {
            std::thread::sleep(Duration::from_millis(200));
            print!("\r=======================>            \x1b[100C");
            _ = stdout().flush();
        });
    });
}
