//! Модуль описывает общее поведение всех устройств.
//! Так же в модуле представлены некоторые типовые реализации конкретных устройств.
//! Так же мы позволяем пользователю реализовывать собственные устройства и использовать
//! их в библиотеки умного дома.

mod socket;
mod thermometer;

pub use socket::SmartSocket;
pub use thermometer::SmartThermometer;

/// Общее поведение для любых устройство умного дома.
pub trait Device {
    /// Функция получения имени устройства.
    fn name(&self) -> &str;

    /// Возвращает потребляемое количество энергии устройством.
    fn power(&self) -> u16;
}

pub trait DisplayableDevice: Device + std::fmt::Display {}

#[cfg(test)]
mod test;
