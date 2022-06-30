use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;
pub use rep_server::ProcessRequest as ProcessRequest;

#[async_trait]
pub trait MessageBusClient
{
    async fn send_request(&self, destination: &str, data: &str) -> Result<String, String>;

    async fn publish(&self, channel: String, message: String);

    async fn subscribe_channel(&self, channel: &str);
}

pub trait ProcessPublisherMessage
{
    fn process_message(&self, message: &str);
}

pub struct ZmqMessageBusClient<T>
where T: rep_server::ProcessRequest + Send + std::marker::Sync + 'static
{
    publisher: Arc<Mutex<publisher::Publisher>>,
    _subscriber: Arc<subscriber::Subscriber>,
    requester: Arc<Mutex<req_client::RequestClient>>,
    _replier: Arc<rep_server::ReplyServer<T>>
}

impl<T: rep_server::ProcessRequest + Send + std::marker::Sync + 'static> ZmqMessageBusClient<T>
{
    pub async fn connect
        <U: ProcessPublisherMessage + Send + std::marker::Sync + 'static>
        (configurations: &config_loader::ConfigLoader,
                         message_processor: T, published_message_processor: U) -> ZmqMessageBusClient<T>
    {
        let identity = configurations.get_string("zmq_message_bus.identity").unwrap();
        let channels_strings = configurations.get_vec("zmq_message_bus.subscription_channels").unwrap();
        let channels = channels_strings.iter().map(|x| x.as_str()).collect();
        let message_bus_address_for_pubs
            = configurations.get_string("zmq_message_bus.address_for_pubs").unwrap();
        let message_bus_address_for_subs
            = configurations.get_string("zmq_message_bus.address_for_subs").unwrap();
        let message_bus_address_for_router
            = configurations.get_string("zmq_message_bus.address_for_router").unwrap();

        // in the identity, it is necessary to distinguish between the requester
        // and the replier since these are separate connections
        let publisher = Arc::new(Mutex::new(publisher::Publisher::new(
                    &message_bus_address_for_subs, false)));
        let _subscriber = Arc::new(subscriber::Subscriber::new(
                    channels, &message_bus_address_for_pubs, false));
        let requester = Arc::new(Mutex::new(req_client::RequestClient::new(
                    &format!("{}{}", &identity, "_requester"),
                             &message_bus_address_for_router)));
        let _replier = Arc::new(rep_server::ReplyServer::new(
                    &format!("{}{}", &identity, "_replier"),
                             message_processor,
                             &message_bus_address_for_router));
        
        tokio::spawn(
        {
            let rep = Arc::clone(&_replier);
            async move
            {
                rep.receive_requests().await;
            }
        });        
        
        tokio::spawn(
        {
            let sub = Arc::clone(&_subscriber);
            async move
            {
                loop
                {
                    published_message_processor.process_message(&sub.receive().await);
                }
            }
        });

        ZmqMessageBusClient{publisher, _subscriber, requester, _replier}       
    }
}

#[async_trait]
impl<T: rep_server::ProcessRequest + Send + std::marker::Sync + 'static> MessageBusClient for ZmqMessageBusClient<T>
{
    async fn send_request(&self, destination: &str, data: &str) -> Result<String, String>
    {
        self.requester.lock().await.send_request(&format!("{}{}", destination, "_replier"), data).await
    }

    async fn publish(&self, channel: String, message: String)
    {
        tokio::spawn(
        {
            let publisher = Arc::clone(&self.publisher);
            async move
            {
                publisher.lock().await.send_string(&channel, &message);
            }
        }); 
    }

    async fn subscribe_channel(&self, channel: &str)
    {
        //TODO
        println!("subscribe_channel not implemented. {} not subscribed.", channel);
    }
}
