//! Простой TCP клиент для взаимодействия с сервером умного устройства.

use std::io::{Read, Write};
use std::net::TcpStream;

/// Реализация простого клиента по взаимодействию с умным устройством через TCP протокол.
pub struct DeviceTcpClient {
    address: String,
}

impl DeviceTcpClient {
    pub fn with_address(address: String) -> Self {
        DeviceTcpClient { address }
    }

    /// Метод отправки команды к TCP серверу умного устройства.
    /// Команда это 1 байт данных, в результате запроса формируется строка ответ от сервера.
    pub fn send_command(&self, command: u8) -> Option<String> {
        let mut stream = TcpStream::connect(&self.address).unwrap();
        stream.write_all(&[command; 1]).unwrap();
        let mut buf = String::new();
        if stream.read_to_string(&mut buf).is_ok() {
            Some(buf)
        } else {
            None
        }
    }
}
