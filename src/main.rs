/*fn main() 
{
    let config_path = "appconfig.toml"; 
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new(config_path);

    let ctx = zmq::Context::new();
    let router = ctx.socket(zmq::ROUTER).unwrap();
    router.bind(&config_loader.get_value("message_router_address").unwrap()).unwrap();

    loop
    {
        let mut message: Vec<Vec<u8>> = router.recv_multipart(0).unwrap();

        let source_address = message[0].clone();
        let dest_address = message[2].clone();

        message[0] = dest_address;
        message[2] = source_address;
        match router.send_multipart(message, 0)
        {
            Ok(success) => success,
            Err(err) => println!("{}", err)
        };
    } 
}
*/
#[tokio::main]
pub async fn main()
{
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new("appconfig.toml");
    let message_bus_address_for_pubs = config_loader.get_value("message_bus_address_for_publishers").unwrap();
    let message_bus_address_for_subs = config_loader.get_value("message_bus_address_for_subscribers").unwrap();
    let message_bus_req_rep_router = config_loader.get_value("message_bus_address_for_router").unwrap();

    let publisher = publisher::Publisher::new(message_bus_address_for_pubs.as_str(), true);
    let subscriber = subscriber::Subscriber::new("", message_bus_address_for_subs.as_str(), true);

    //let message_router = message_router::MessageRouter::new(message_bus_req_rep_router);
    let message_bus = 
        tokio::spawn(async move
        {
            loop
            {

              let message = subscriber.receive_raw().await;
              publisher.send_string("", &message.as_str());
            }
        });
    let req_rep_router = 
        tokio::spawn(async move
        {
            loop
            {
                println!("To implement router");
                std::thread::sleep(std::time::Duration::new(1, 0));
            }
        });

    message_bus.await.unwrap();
    req_rep_router.await.unwrap();
         
}
