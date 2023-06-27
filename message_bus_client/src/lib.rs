use std::thread;
pub use rep_server::ProcessRequest as ProcessRequest;

pub trait MessageBusClient
{
    fn send_request(&self, destination: &str, data: &str) -> Result<String, String>;

    fn publish(&self, channel: String, message: String);

    fn publish_bytes(&self, channel: String, message: Vec<u8>);

    fn subscribe_channel(&self, channel: &str);
}

pub trait ProcessPublisherMessage
{
    fn process_message(&self, message: &str);
}

pub struct ZmqMessageBusClient
{
    publisher: publisher::Publisher,
    requester: req_client::RequestClient,
    
    _replier_handle: thread::JoinHandle<()>,
    _subscriber_handle: thread::JoinHandle<()>  
}

impl ZmqMessageBusClient
{
    pub fn connect
        <U: ProcessPublisherMessage + Send + std::marker::Sync + 'static,
        T: rep_server::ProcessRequest + Send + std::marker::Sync + 'static>
        (configurations: &config_loader::ConfigLoader,
                         message_processor: T, published_message_processor: U) -> ZmqMessageBusClient
    {
        let identity = configurations.get_string("zmq_message_bus.identity").unwrap();
        let channels_strings = configurations.get_vec("zmq_message_bus.subscription_channels").unwrap();
        let message_bus_address_for_pubs
            = configurations.get_string("zmq_message_bus.address_for_pubs").unwrap();
        let message_bus_address_for_subs
            = configurations.get_string("zmq_message_bus.address_for_subs").unwrap();
        let message_bus_address_for_router
            = configurations.get_string("zmq_message_bus.address_for_router").unwrap();

        // in the identity, it is necessary to distinguish between the requester
        // and the replier since these are separate connections
        let publisher = 
            publisher::Publisher::new(
                &message_bus_address_for_pubs,
                false);

        let requester =
            req_client::RequestClient::new(
                &format!("{}{}", &identity, "_requester"),
                &message_bus_address_for_router);
        let replier = rep_server::ReplyServer::new(
            &format!("{}{}", &identity, "_replier"),
                             message_processor,
                             &message_bus_address_for_router);
        
        let _replier_handle = thread::spawn(
        {
            move ||
            {
                replier.receive_requests();
            }
        });        
        
        let _subscriber_handle = thread::spawn(
        {
            move ||
            {
                let channels = channels_strings.iter().map(|x| x.as_str()).collect();
                let subscriber = 
                    subscriber::Subscriber::new(
                        channels,
                        &message_bus_address_for_subs,
                        false);
                loop
                {
                    published_message_processor.process_message(&subscriber.receive());
                }
            }
        });

        ZmqMessageBusClient{publisher, requester, _replier_handle, _subscriber_handle}       
    }
}

impl MessageBusClient for ZmqMessageBusClient
{
    fn send_request(&self, destination: &str, data: &str) -> Result<String, String>
    {
        self.requester.send_request(&format!("{}{}", destination, "_replier"), data)
    }

    fn publish(&self, channel: String, message: String)
    {
        self.publisher.send_string(&channel, &message);
    }

    fn publish_bytes(&self, channel: String, message: Vec<u8>)
    {
        self.publisher.send_bytes(&channel, &message);
    }

    fn subscribe_channel(&self, channel: &str)
    {
        //TODO
        println!("subscribe_channel not implemented. {} not subscribed.", channel);
    }
}
