mod server;
mod machine;

use server::Server;
use async_std::task;

fn main() {
    task::block_on(try_main());

    println!("Terminated");
}

async fn try_main() {
    let mut server = Server::new();
    let server_handle = task::spawn(async {});

    // wait for machine stopped
    // wait for server stopped
}
