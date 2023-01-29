use std::{net::UdpSocket, thread, time::Duration};

use smart_house::device::SmartThermometer;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:3001").unwrap();
    let server_addr = "127.0.0.1:3000";

    let mut thermometer = SmartThermometer::with_name("Термометр №1");

    socket.connect(server_addr)?;

    let mut buf = [0; 4];

    loop {
        thread::sleep(Duration::from_secs(3));

        socket.send_to([1; 1].as_slice(), "127.0.0.1:3000")?;

        socket.recv(&mut buf)?;

        thermometer.set_temperature(f32::from_be_bytes(buf));
        println!("Report from thermometer: {}", thermometer);
    }
}
