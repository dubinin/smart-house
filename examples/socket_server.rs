use smart_house::device::Device;
use smart_house::device::DeviceTcpServer;
use smart_house::device::SmartSocket;

fn main() {
    let mut socket = SmartSocket::with_name("розетка №1");
    if let Ok(server) = DeviceTcpServer::bind("0.0.0.0:3000") {
        socket.start_server(server);
    }
}
