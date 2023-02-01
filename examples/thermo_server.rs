use std::{
    net::UdpSocket,
    sync::{Arc, RwLock},
    thread,
};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:3000").unwrap();

    let value = Arc::new(RwLock::<f32>::new(0.0));

    let value_for_write = Arc::clone(&value);

    thread::spawn(move || {
        let mut user_input = String::new();

        loop {
            user_input.clear();
            std::io::stdin().read_line(&mut user_input).unwrap();

            match user_input.trim().parse::<f32>() {
                Ok(input) => {
                    // Изменить значение для value.
                    println!("Change value to: {}", input);
                    *value_for_write.write().unwrap() = input;
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
        socket.send_to(value.read().unwrap().to_be_bytes().as_slice(), sender)?;
    }
}
