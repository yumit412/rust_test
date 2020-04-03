use async_std::task;
use std::time::Duration;

pub struct Server{
    handle : async_std::task::JoinHandle<()>,
}

impl Server {
    pub fn new() -> Self{
        Self{
            handle: task::spawn(async {}),
        }
    }

    pub fn start(&mut self) -> &Self{
        let future = async {
            println!("server is startd...");

            let dur = Duration::from_secs(10);
            task::sleep(dur).await;
            
            println!("server is closed");
            };
        self.handle = task::spawn(future);
        
        self
    }

    pub async fn stop(self) {
        self.handle.await;


        println!("Server has Stopped");

    }
    
}