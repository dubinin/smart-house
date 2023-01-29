use std::{net::UdpSocket, thread};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:3000").unwrap();

    let value: f32 = 18.0;

    thread::spawn(move || {
        let mut user_input = String::new();

        loop {
            user_input.clear();
            std::io::stdin().read_line(&mut user_input).unwrap();

            match user_input.trim().parse::<f32>() {
                Ok(input) => {
                    println!("Change value to: {}", input);
                    // TODO: Изменить значение для value.
                }
                Err(err) => {
                    println!("Value parse error: {}", err);
                    continue;
                }
            }
        }
    });

    loop {
        let mut buf = [0; 1];
        let (size, sender) = socket.recv_from(&mut buf)?;
        println!("Received {} bytes from {}.", size, sender);

        // Отправим текущее значение температуры.
        socket.send_to(value.to_be_bytes().as_slice(), sender)?;
    }
}
