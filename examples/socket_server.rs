use smart_house::device::Device;
use smart_house::device::SmartSocket;

fn main() {
    let socket = SmartSocket::with_name_server("розетка №1", "0.0.0.0:3000");
    socket.start_server();
}
