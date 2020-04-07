use modbus::{Client, Coil};
use modbus::tcp;

struct ModbusDevice {
}

fn test() {
    let mut cfg = tcp::Config::default();
    let mut client = tcp::Transport::new_with_cfg("127.0.0.1", cfg).unwrap();
    assert!(client.write_single_coil(0, Coil::On).is_ok());
}