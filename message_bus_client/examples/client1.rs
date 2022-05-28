use std::sync::{Arc};
use std::thread;

use zmq_message_bus_client::MessageBusClient;

pub struct MessageProcessor {}

const message_bus_address_for_pubs: &str = "tcp://127.0.0.1:13031";
const message_bus_address_for_subs: &str = "tcp://127.0.0.1:13032";
const message_bus_address_for_router: &str = "tcp://127.0.0.1:13033";


impl rep_server::ProcessRequest for MessageProcessor
{
    fn process_message(&self, input: &str) -> String
    {
        println!("{}", input);
        input.to_string()
    }
}

#[tokio::main]
pub async fn main()
{
    let message_bus_client_1 = MessageBusClient::connect("client1",
                                                       "",
                                                       message_bus_address_for_pubs,
                                                       message_bus_address_for_subs,
                                                       message_bus_address_for_router,
                                                       MessageProcessor{}
                                                       ).await;
    //let mb_listen_1_future = message_bus_client_1.listen();
    
    
    let send_inputted_strings_future = send_message_loop(&message_bus_client_1).await; 
    //tokio::join!(mb_listen_1_future, send_inputted_strings_future);
    //send_inputted_strings_future.await;
    //mb_listen_1_future.await;
}

async fn send_message_loop(message_bus_client: &MessageBusClient<MessageProcessor>)
{
    loop
    {   
        let mut line = String::new();
        println!("Enter text to send:\n");

        std::io::stdin().read_line(&mut line).unwrap();
        message_bus_client.send_request("client2", &line).await.unwrap();
    }
}
