//! Модуль реализующий сетевое взаимодействие для умных устройств.

mod client;
mod server;

pub use client::DeviceTcpClient;
pub use server::DeviceTcpServer;
