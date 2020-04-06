use std::net::Ipv4Addr;
use async_std::task;
use async_std::task::JoinHandle;

pub struct Server{
    addr: Option<Ipv4Addr>,
    handle: Option<JoinHandle<()>>,
}

impl Server {
    pub fn new() -> Self{
        Self{
            addr: None,
            handle: None,
        }
    }

    pub fn listen(&mut self) -> &Self{
        self
    }

    pub fn stop(self) {
        if let Some(handle) = self.handle {
            task::block_on(async {handle.await});
            println!("wait for task");
        } else {
            // stop immediately
        }
        println!("Server Stopped");
    }

    pub fn start(&mut self) {
        if let Some(_) = self.addr {
        } else {
            let addr = Some(Ipv4Addr::new(127, 0, 0, 1));
            self.addr = addr;
            println!("No address specified.");
            println!("Addr {}", addr.unwrap());
        }

        if let Some(handle) = &self.handle{
            println!("Already have a task for server");
        } else {
            let handle = task::spawn(async {println!("task startd")});
            self.handle = Some(handle);
        }
        println!("Server start");
    }
}