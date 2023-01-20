//! Серверная часть реализации сетевого взаимодействия с умными устройствами.

use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub type BindResult = Result<DeviceTcpServer, BindError>;

/// Реализация TCP сервера для удаленного взаимодействия с умным устройством.
pub struct DeviceTcpServer {
    listener: TcpListener,
}

#[derive(Debug)]
pub enum BindError {
    Io(std::io::Error),
}

impl DeviceTcpServer {
    pub fn bind<Addrs>(addrs: Addrs) -> BindResult
    where
        Addrs: ToSocketAddrs,
    {
        match TcpListener::bind(addrs) {
            Ok(listener) => Ok(DeviceTcpServer { listener }),
            Err(io) => Err(BindError::Io(io)),
        }
    }

    pub fn incoming(&self) -> impl Iterator<Item = TcpStream> + '_ {
        self.listener.incoming().filter_map(|con| con.ok())
    }
}
