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

    /// Функция создает отчет по устройству.
    fn report(&self) -> String;
}

/// Реализация Display трейта для любых устройств.
impl std::fmt::Display for dyn Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.report())
    }
}
