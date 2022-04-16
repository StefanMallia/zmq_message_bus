pub struct MessageBusClient<T>
where T: rep_server::ProcessRequest
{
    pub publisher: publisher::Publisher,
    pub subscriber: subscriber::Subscriber,
    pub requester: req_client::RequestClient,
    pub replier: rep_server::ReplyServer<T>
}

impl <T: rep_server::ProcessRequest> MessageBusClient<T>
{
    pub async fn connect(identity: &str,
                         subscriber_channel: &str,
                         message_bus_address_for_pubs: &str,
                         message_bus_address_for_subs: &str,
                         message_bus_address_for_router: &str,
                         message_processor: T) -> MessageBusClient<T>
    {
        let publisher = publisher::Publisher::new(&message_bus_address_for_pubs, false);
        let subscriber = subscriber::Subscriber::new(subscriber_channel, message_bus_address_for_subs, false);
        let requester = req_client::RequestClient::new(identity, message_bus_address_for_router);
        let replier = rep_server::ReplyServer::new(identity, message_processor, message_bus_address_for_router);

        MessageBusClient{publisher, subscriber, requester, replier}       
    }
}
