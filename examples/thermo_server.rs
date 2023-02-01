use std::{net::UdpSocket, thread, sync::{Arc, atomic::{AtomicI32, Ordering}}};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:3000").unwrap();

    let value = Arc::new(AtomicI32::new(0));

    let value_for_write = Arc::clone(&value);

    thread::spawn(move || {
        let mut user_input = String::new();

        loop {
            user_input.clear();
            std::io::stdin().read_line(&mut user_input).unwrap();

            match user_input.trim().parse::<i32>() {
                Ok(input) => {
                    // Изменить значение для value.
                    println!("Change value to: {}", input);
                    value_for_write.store(input, Ordering::SeqCst);
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
        socket.send_to(value.load(Ordering::SeqCst).to_be_bytes().as_slice(), sender)?;
    }
}
