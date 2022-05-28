use std::sync::{Arc};
use futures::lock::Mutex;

pub struct MessageBusClient<T>
where T: rep_server::ProcessRequest + Send + std::marker::Sync + 'static
{
    publisher: Arc<Mutex<publisher::Publisher>>,
    subscriber: Arc<Mutex<subscriber::Subscriber>>,
    requester: Arc<Mutex<req_client::RequestClient>>,
    replier: Arc<rep_server::ReplyServer<T>>
}

impl <T: rep_server::ProcessRequest + Send + std::marker::Sync + 'static> MessageBusClient<T>
{
    pub async fn connect(identity: &str,
                         subscriber_channel: &str,
                         message_bus_address_for_pubs: &str,
                         message_bus_address_for_subs: &str,
                         message_bus_address_for_router: &str,
                         message_processor: T) -> MessageBusClient<T>
    {
        let publisher = Arc::new(Mutex::new(publisher::Publisher::new(&message_bus_address_for_pubs, false)));
        let subscriber = Arc::new(Mutex::new(subscriber::Subscriber::new(subscriber_channel, message_bus_address_for_subs, false)));
        let requester = Arc::new(Mutex::new(req_client::RequestClient::new(identity, message_bus_address_for_router)));
        let replier = Arc::new(rep_server::ReplyServer::new(identity, message_processor, message_bus_address_for_router));
        
        //replier.lock().await.receive_requests().await;

        tokio::spawn(
        {
            let rep = Arc::clone(&replier);
        
       
            async move
            {
                rep.receive_requests().await;

            }
        }
            
        );

        
        tokio::spawn(
        {
            let sub = Arc::clone(&subscriber);
        
       
            async move
            {
                sub.lock().await.receive().await;
            }
        }
            
        );



        MessageBusClient{publisher, subscriber, requester, replier}       
    }

    pub async fn send_request(&self, destination: &str, data: &str) -> Result<String, String>
    {
        self.requester.lock().await.send_request(destination, data).await
    }

    pub async fn publish(&self, channel: &str, message: &str)
    {
        self.publisher.lock().await.send_string(channel, message);
    }
    /*

    pub async fn listen(&self)
    {

        let replier_listen_future = self.replier.receive_requests();
        let subscriber_listen_future = self.subscriber.receive();

        tokio::join!(subscriber_listen_future, replier_listen_future);
    }*/
}
