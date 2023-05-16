// USE THIS TO RUN THE CODE WITH ONLY 1 THREAD:
// cargo run -j 1 

mod tsh;
use std::io::Write;

fn main() {

    let mut shell = tsh::SimpleShell::new();

    print!("tsh> ");
    std::io::stdout().flush().expect("Failed to flush stdout.");

    // While there is a line to read
    let mut input = String::new();
    while match std::io::stdin().read_line(&mut input) {
        Ok(_) => true,
        Err(e) => {
            println!("error: {e}");
            false
        }
    } {
        // If there is some input other than next line
        if input != "\n" {
            shell.parse_command(input);
            if shell.is_quit() {
                return;
            }
            else{
                shell.exec_command().expect("Failed to execute command");
            }
        }

        // Reset input string
        input = String::new();

        print!("tsh> ");
        std::io::stdout().flush().expect("Failed to flush stdout.");
    }
}
