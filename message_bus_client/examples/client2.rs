use zmq_message_bus_client::ZmqMessageBusClient;
use zmq_message_bus_client::ProcessRequest;

pub struct MessageProcessor {}

impl ProcessRequest for MessageProcessor
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
    let configurations = config_loader::ConfigLoader::new("examples/appconfig_client2.toml");
    let a = ZmqMessageBusClient::connect(&configurations,
                                 MessageProcessor{}
                                 ).await;
}

