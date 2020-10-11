# I2T Streams MQTT Gateway

## Preparation
Install rust if you don't have it already, find the instructions here https://www.rust-lang.org/tools/install

Make sure you also have the build dependencies installed, if not run:  
`sudo apt install build-essential`  
`sudo apt install pkg-config`  
`sudo apt install libssl-dev`  
`sudo apt update`  


## Installing the MQTT broker
In case you don't have a MQTT broker running yet, you can run one locally with Mosquitto:  
`sudo apt install mosquitto`  
`mosquitto`  


## Installing the streams-mqtt-gateway

Download the Repository:  

`git clone https://github.com/iot2tangle/streams-mqtt-gateway.git`
  
Configure the streams-gateway:  

`nano config.json`  
 
Set the *device_name* to the value specified in the configuration file of the Device.  
Set the *borker_ip, broker_port* to match the location on the broker, (default MQTT port is 1883).  
Change *topic, node, mwm, local_pow* if needed 



  
## Runnig the Examples:  
  
Run the streams-gateway:  

`cargo run --release`  

This starts the server which will forward messages from the devices to the Tangle  
  
The Output will be something like this:  

`>> Starting.... `  
`>> Channel root: "ab3de895ec41c88bd917e8a47d54f76d52794d61ff4c4eb3569c31f619ee623d0000000000000000"`  
  
`>> To read the messages copy the channel root into http://iot2tangle.link/ `  
  
`>> Listening for topic iot2tangle on http://localhost:1883`  
 

To send data to the server you can use cURL, make sure the ip and port are the same as in the config.json file, and that they point to the broker:  
`mosquitto_pub -h localhost -p 1883 -t "iot2tangle" -m '{ "iot2tangle": [ { "sensor": "Gyroscope", "data": [ { "x": "4514" }, { "y": "244" }, { "z": "-1830" } ] }, { "sensor": "Acoustic", "data": [ { "mp": "1" } ] } ], "device": "DEVICE_ID", "timestamp": "" }'`