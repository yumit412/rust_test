use async_std::task;
use futures::executor;
mod server;
use server::Server;

struct App{}

impl App{
    pub fn new() -> Self{
        Self{}
    }
}

async fn handle_tcp() {
    println!("listen tcp");
}


async fn try_main() {
    println!("Main Task");
    let app= App::new();

    let mut handles = vec![];

    let handle = task::spawn(handle_tcp());
    handles.push(handle);

    // start tcp server
    let mut server = Server::new();
    server.start();
    let future = async {println!("futures has excuted...")};
    let handle2 = task::spawn(future);
    handles.push(handle2);

    server.stop().await;
    
    // create task that handle TcpStrem
    
    
    for handle in handles{
        handle.await;
    }

    
}

fn main() {
    executor::block_on(try_main());
    
    println!("Main Block Terminated");
}
