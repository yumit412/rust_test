use modbus::{Client, Coil};
use modbus::tcp;

enum IO_TYPE{
    input,
    output,
    io,
}

struct IO<IO_TYPE> {
    name: String,
    address: u32,
    value: u8,
}

impl IO_TYPE {
    fn new(name: String) -> Self{
        Self{
            name:name,
            address: None,
            Value: None,
        }
    }
}

struct ModbusDevice {
}

fn construct_io_map {
    let io1 = IO<IO_TYPE::input>::new("io1".to_string());
    
}

fn test() {
    let mut cfg = tcp::Config::default();
    let mut client = tcp::Transport::new_with_cfg("127.0.0.1", cfg).unwrap();
    assert!(client.write_single_coil(0, Coil::On).is_ok());
}