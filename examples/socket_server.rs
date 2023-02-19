use smart_house::device::DeviceTcpServer;
use smart_house::device::SmartSocket;

fn main() {
    let socket = SmartSocket::with_name("розетка №1");
    if let Ok(ref mut server) = DeviceTcpServer::bind("0.0.0.0:3000", socket) {
        server.start();
    }
}
