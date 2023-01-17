use zmq_message_bus_client::ZmqMessageBusClient;
use zmq_message_bus_client::ProcessRequest;
use async_trait::async_trait;

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
    let configurations = config_loader::ConfigLoader::new("examples/appconfig_client2.toml");
    let _message_bus_client_2 = ZmqMessageBusClient::connect(&configurations,
                                                            MessageProcessor{},
                                                            PublishedMessageProcessor{}
                                                           ).await;
    
}

