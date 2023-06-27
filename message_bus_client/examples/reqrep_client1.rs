use zmq_message_bus_client::ZmqMessageBusClient;
use zmq_message_bus_client::MessageBusClient;
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


struct PublishedMessageProcessor
{}

impl zmq_message_bus_client::ProcessPublisherMessage for PublishedMessageProcessor
{
    fn process_message(&self, input: &str)
    {
        println!("{}", input);
    }
}

pub fn main()
{
    let configurations = config_loader::ConfigLoader::new("examples/appconfig_client1.toml");
    let message_bus_client_1 = ZmqMessageBusClient::connect(&configurations,
                                                            MessageProcessor{},
                                                            PublishedMessageProcessor{}
                                                           );
    
    send_message_loop(&message_bus_client_1); 
}

fn send_message_loop(message_bus_client: &ZmqMessageBusClient)
{
    let mut increment = 0;
    loop
    {   
        increment = increment + 1;

        let message = format!("Message from client1 {}", increment);

        let reply = message_bus_client.send_request("client2", &message).unwrap();
        println!("Reply from client 2: {}", reply);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
