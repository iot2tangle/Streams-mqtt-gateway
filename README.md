# I2T Streams MQTT Gateway

## Preparation
Install rust if you don't have it already, find the instructions here https://www.rust-lang.org/tools/install

Make sure you also have the build dependencies installed, if not run:
```
sudo apt update; sudo apt install build-essential pkg-config libssl-dev cmake
```

## Installing the MQTT broker
In case you don't have a MQTT broker running yet, you can run one locally with Mosquitto:
```
sudo apt install mosquitto
```

If you don't want to set authentication for users run:  

```
mosquitto
```

To enable authentication through usernames and passwords, follow the steps in [this guide](http://www.steves-internet-guide.com/mqtt-username-password-example/) 

## Installing the streams-mqtt-gateway

Download the Repository:  
```
git clone https://github.com/iot2tangle/streams-mqtt-gateway.git
```

Configure the streams-gateway:  

```
nano config.json
```
 
Set the *whitelisted_device_ids* to include all the device IDs specified in their respective configuration files.

Leave *username, password* empty ("") if your borker does not require authentication.  
Set the *username, password* if you are connecting to an authenticaded broker.    
Set the *borker_ip, broker_port* to match the location on the broker, (default MQTT port is 1883).  
Change *topic, node, mwm, local_pow* if needed 



  
## Runnig the Gateway:  
  
Run the streams-gateway:  

```
cargo run --release
```

This starts the server which will forward messages from the devices to the Tangle  
  
The Output will be something like this:  

```
>> Starting....
>> Channel root: "47d504e1a825e142dd899dda81ff787c7cfad3b83977feec3545eaef4315c8a50000000000000000:fd93e57d937910f429cdd211"
  
>> To read the messages copy the channel root into https://explorer.iot2tangle.io/
  
>> Listening for topic iot2tangle on http://localhost:1883
``` 

To send data to the server you can use cURL, make sure the ip and port are the same as in the config.json file, and that they point to the broker:  

```
mosquitto_pub -h localhost -p 1883 -u -P  -t "iot2tangle" -m '{ "iot2tangle": [ { "sensor": "Gyroscope", "data": [ { "x": "4514" }, { "y": "244" }, { "z": "-1830" } ] }, { "sensor": "Acoustic", "data": [ { "mp": "1" } ] } ], "device": "DEVICE_ID_1", "timestamp": "" }'
```

Set values for *-u* *-P* to spsecify the username and the password for an authenticated broker  
