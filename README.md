# Библиотека Умный Дом

## Примеры

- `cargo run --example basic` - базовый пример формирование отчета для умного дома из нескольких комнат.
- `cargo run --example custom_device` - пример использования библиотеки с пользовательскими устройствами.
- `cargo run --example socket_server` - пример использования TCP сервера для умной розетки.
- `cargo run --example socket_client` - пример использование TCP клиента для взаимодействия с умной розеткой. Для правильной работы сначала нужно запустить пример сервера `cargo run --example socket_server`.
- `cargo run --example thermo_client` - пример использование UDP клиента для взаимодействия с умным термометром. Для правильной работы сначала нужно запустить пример сервера `cargo run --example thermo_server`. На стороне сервера есть возможность изменять значение введя его в консоли, поддерживается число с плавающей точкой.
