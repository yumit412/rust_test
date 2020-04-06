use async_std::task;
use async_std::task::JoinHandle;
use std::sync::Arc;

#[derive(Clone, Debug)]
struct State {}

trait Device {}

#[derive(Clone)]
pub struct Machine{
    machines: Vec<Arc<dyn Device>>,// modbus devices..
    state: State,
}

impl Machine{
    pub fn new() -> Self{
        Self{
            machines: Vec::new(),
            state: State{},
        }
    }

    pub fn start(&mut self) {
        println!("start");
    }


    pub fn stop(&self) {
        println!("start");
    }
}

struct ModbusDevice {
    name:String,
    handle: Option<JoinHandle<()>>,
}

impl ModbusDevice {
    pub fn new(name:String) -> Self{
        Self{
            name: name,
            handle: None,
        }
    }

    pub fn run() {
        let handle = task::spawn(async {});
    }

    fn try_run(&self) {
        loop{
            // polling
        }
    }

    fn update_device(&mut self) {

    }

    pub fn stop(&mut self) {
        match &self.handle {
            None => {},
            Some(handle) => {
                //task::block_on(handle);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_test() {
        let device = ModbusDevice::new("test device".to_string());

    }
}