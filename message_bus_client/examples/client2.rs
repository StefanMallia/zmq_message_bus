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
    let message_bus_client_1 = MessageBusClient::connect("client2",
                                                       "",
                                                       message_bus_address_for_pubs,
                                                       message_bus_address_for_subs,
                                                       message_bus_address_for_router,
                                                       MessageProcessor{}
                                                       ).await;
    //message_bus_client_1.listen().await;
    
    
}


