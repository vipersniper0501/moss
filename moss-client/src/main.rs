
mod structures;
mod linux;

/// Gets config from a local file then runs the perform_checks
/// function to begin
fn local_mode(path: &str) {

}

/// Gets config from a remote server then runs the perform_checks
/// function to begin
fn remote_mode(address: &str) {

}

fn print_usage() {
    println!("Usage:\tmoss-client [-L | --local] <path to local config>\n\tOr\n\tmoss-client [-R | --remote] <IP/Url of remote server>");
}

/// Attempts to form connection with server.
/// Connection Fails:
///     Exit and report failure
/// Connection Succeeds: 
///     Deserialize data
///     Data should contain the instructions for the checks
/// Main loop:
///     - Perform Checks
///     - Post checks to server
///     - repeat
fn main() {
    if std::env::args().len() != 3 {
        print_usage();
        std::process::exit(1);
    } 

    let mode = match std::env::args().nth(1) {
        Some(x) => x,
        None => {
            println!("Missing arguments.");
            // Should never reach this but who knows...
            print_usage();
            std::process::exit(1);
        },
    };

    let mode_argument = match std::env::args().nth(2) {
        Some(x) => x,
        None => {
            println!("Missing mode argument.");
            print_usage();
            std::process::exit(1);
        }
    };

    match mode.as_str() {
        "-L" | "--local" => {
            println!("Running moss-client in Local Mode.");
            local_mode(mode_argument.as_str());
        },
        "-R" | "--remote" => {
            println!("Running moss-client in Remote Mode.");
            remote_mode(mode_argument.as_str());
        },
        _ => {
            println!("Invalid arguments");
            print_usage();
            std::process::exit(1);
        }
    }

}
