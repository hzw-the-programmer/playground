sudo apt-get install mosquitto mosquitto-clients
mosquitto_sub -h localhost -t test
mosquitto_pub -h localhost -t test -m "hello"
