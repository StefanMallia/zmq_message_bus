use zmq_message_bus_client::ZmqMessageBusClient;
use zmq_message_bus_client::MessageBusClient;
use zmq_message_bus_client::ProcessRequest;
use async_trait::async_trait;

pub struct MessageProcessor {}

impl ProcessRequest for MessageProcessor
{
    fn process_message(&self, input: &str) -> String
    {
        println!("{}", input);
        input.to_string()
    }
}


struct PublishedMessageProcessor
{}

#[async_trait]
impl zmq_message_bus_client::ProcessPublisherMessage for PublishedMessageProcessor
{
    async fn process_message(&self, input: &str)
    {
        println!("{}", input);
    }
}

#[tokio::main]
pub async fn main()
{
    let configurations = config_loader::ConfigLoader::new("examples/appconfig_client1.toml");
    let message_bus_client_1 = ZmqMessageBusClient::connect(&configurations,
                                                            MessageProcessor{},
                                                            PublishedMessageProcessor{}
                                                           ).await;
    
    let _send_inputted_strings_future = send_message_loop(&message_bus_client_1).await; 
}

async fn send_message_loop(message_bus_client: &ZmqMessageBusClient<MessageProcessor>)
{
    let mut increment = 0;
    loop
    {   
        increment = increment + 1;

        let message = format!("Message from client1 {}", increment);

        let reply = message_bus_client.send_request("client2", &message).await.unwrap();
        println!("Reply from client 2: {}", reply);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
