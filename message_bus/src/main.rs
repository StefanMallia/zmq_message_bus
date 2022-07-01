#[tokio::main]
pub async fn main()
{
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new("appconfig.toml");
    let message_bus_address_for_pubs = config_loader.get_string("message_bus_address_for_publishers").unwrap();
    let message_bus_address_for_subs = config_loader.get_string("message_bus_address_for_subscribers").unwrap();
    let message_bus_req_rep_router = config_loader.get_string("message_bus_address_for_router").unwrap();

    let publisher = publisher::Publisher::new(message_bus_address_for_subs.as_str(), true);
    let subscriber = subscriber::Subscriber::new(vec![""], message_bus_address_for_pubs.as_str(), true);

    let message_bus = 
        tokio::spawn(async move
        {
            loop
            {
              let message = subscriber.receive_raw().await;
              publisher.send_string("", &message.as_str());
            }
        });
    let message_router = zmq_message_router::MessageRouter::new(&message_bus_req_rep_router);
    let req_rep_router = 
        tokio::spawn(async move
        {
            message_router.route_messages().await;
        });

    message_bus.await.unwrap();
    req_rep_router.await.unwrap();
}
