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
        println!("This is the received message from publisher: {}", input);
    }
}

pub fn main()
{
    let configurations = config_loader::ConfigLoader::new("examples/appconfig_client2.toml");
    let message_bus_client_2 = ZmqMessageBusClient::connect(&configurations,
                                                            MessageProcessor{},
                                                            PublishedMessageProcessor{}
                                                           );
    message_bus_client_2.run();
}
