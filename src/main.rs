use std::process::Command;

fn main() {
    println!("Hello, world!");
}

fn is_online(server_ip: &str) -> bool {
    true
}

fn shutdown(server_ip: &str) {

}

fn boot(server_mac: &str) {
    let cmd = Command::new("wakeonlan")
        .arg(server_mac)
        .output()
        .expect("Cannot execute command.");

    let output = cmd.stdout;

}
