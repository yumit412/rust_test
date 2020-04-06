mod server;
mod machine;

use server::Server;
use machine::Machine;
use async_std::task;

fn main() {
    task::block_on(try_main());

    println!("Terminated");
}

async fn try_main() {
    let mut server = Server::new();
    let mut machine = Machine::new();
    
    server.start();
    machine.start();
    
    machine.stop();
    server.stop();
}
