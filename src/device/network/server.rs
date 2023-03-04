//! Серверная часть реализации сетевого взаимодействия с умными устройствами.

use crate::device::Device;
use std::io::{Read, Write};
use std::net::{TcpListener, ToSocketAddrs};
use thiserror::Error;

pub type BindResult<T> = Result<DeviceTcpServer<T>, BindError>;

/// Реализация TCP сервера для удаленного взаимодействия с умным устройством.
pub struct DeviceTcpServer<T: Device> {
    listener: TcpListener,
    device: T,
}

#[derive(Error, Debug)]
pub enum BindError {
    #[error("IO Error.")]
    Io(#[from] std::io::Error),
}

impl<T: Device> DeviceTcpServer<T> {
    pub fn bind<Addrs>(addrs: Addrs, device: T) -> BindResult<T>
    where
        Addrs: ToSocketAddrs,
    {
        match TcpListener::bind(addrs) {
            Ok(listener) => Ok(DeviceTcpServer { listener, device }),
            Err(io) => Err(BindError::Io(io)),
        }
    }

    pub fn start(&mut self) {
        for mut stream in self.listener.incoming().filter_map(|con| con.ok()) {
            println!(
                "Recieve command from client with ip: {}.",
                stream.peer_addr().unwrap()
            );

            let mut buf = [0; 1];
            if stream.read_exact(&mut buf).is_ok() {
                let response = &self.device.execute(buf[0]);
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    }
}
