use smart_house::device::DeviceTcpClient;

fn main() {
    let client = DeviceTcpClient::with_address(String::from("127.0.0.1:3000"));

    let mut user_input = String::new();

    println!(
        "Welcome to smart socket TCP client!\n
To switch smart socket type commands: 1 (for power on) or 0 (power off).
To display socket report type: 2.
For exit type: exit."
    );

    loop {
        user_input.clear();
        std::io::stdin().read_line(&mut user_input).unwrap();

        let command = user_input.trim();

        if command == "exit" {
            break;
        }

        match command.parse::<u8>() {
            Ok(command) => {
                if let Some(response) = client.send_command(command) {
                    println!("Execute command {}: {}", command, response);
                }
            }
            Err(err) => {
                println!("Command parse error: {}", err);
                continue;
            }
        }
    }
}
