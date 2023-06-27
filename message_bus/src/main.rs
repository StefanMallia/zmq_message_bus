use std::thread;


pub fn main()
{
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new("appconfig.toml");
    rust_log::setup_logger(None).unwrap();
    let message_bus_address_for_pubs = config_loader.get_string("message_bus_address_for_publishers").unwrap();
    let message_bus_address_for_subs = config_loader.get_string("message_bus_address_for_subscribers").unwrap();
    let message_bus_req_rep_router = config_loader.get_string("message_bus_address_for_router").unwrap();

    let publisher = publisher::Publisher::new(message_bus_address_for_subs.as_str(), true);
    let subscriber = subscriber::Subscriber::new(vec![""], message_bus_address_for_pubs.as_str(), true);

    let message_bus = 
        thread::spawn(move ||
        {
            loop
            {
              let message = subscriber.receive_raw();
              publisher.send_string("", &message.as_str());
            }
        });
    let message_router = zmq_message_router::MessageRouter::new(&message_bus_req_rep_router);
    let req_rep_router = 
        thread::spawn(move ||
        {
            message_router.route_messages();
        });

    message_bus.join().unwrap();
    req_rep_router.join().unwrap();
}
