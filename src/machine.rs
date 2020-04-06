use async_std::task;
use async_std::task::JoinHandle;

#[derive(Clone, Debug)]
struct Machine{
    // modbus devices..
    // 
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