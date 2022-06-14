use zmq_message_bus_client::ZmqMessageBusClient;
use zmq_message_bus_client::ProcessRequest;

pub struct MessageProcessor {}

impl ProcessRequest for MessageProcessor
{
    fn process_message(&self, input: &str) -> String
    {
        println!("{}", input);
        format!("This is the reply to '{}'", input)
    }
}


struct PublishedMessageProcessor
{}
impl zmq_message_bus_client::ProcessPublisherMessage for PublishedMessageProcessor
{
    fn process_message(&self, input: &str)
    {
        println!("{}", input);
    }
}

#[tokio::main]
pub async fn main()
{
    let configurations = config_loader::ConfigLoader::new("examples/appconfig_client2.toml");
    let message_bus_client_1 = ZmqMessageBusClient::connect(&configurations,
                                                            MessageProcessor{},
                                                            PublishedMessageProcessor{}
                                                           ).await;
    
    let _send_inputted_strings_future = send_message_loop(&message_bus_client_1).await; 
}

async fn send_message_loop(message_bus_client: &ZmqMessageBusClient<MessageProcessor>)
{
    loop
    {   
        //let mut line = String::new();
        //println!("Enter text to send:\n");

        //std::io::stdin().read_line(&mut line).unwrap();
        //message_bus_client.send_request("client1", "Hello from client2!").await.unwrap();
    }
}
