# ZMQ Message Bus

A message bus application that supports pub-sub and request-response messaging. The message bus application is contained in message_bus. A library that can be used by client applications is in message_bus_client.

The run_message_bus.sh script can be run together with run_pub_sub_example.sh or run_req_rep_example.sh as a demonstration of how this project can be used.

Note: Requires zmq library and headers to be installed.
```bash
apt install libzmq3-dev
```
