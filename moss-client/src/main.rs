
mod structures;
mod linux;

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

}
