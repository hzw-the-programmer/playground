const mqtt = require('mqtt')

if (process.argv.length < 5) {
    console.log(`${process.argv[0]} ${process.argv[1]} url username password`)
    process.exit()
}

const url = process.argv[2]
const username = process.argv[3]
const password = process.argv[4]

const client = mqtt.connect(url, {username, password})
client.on('connect', () => {
    console.log('connected')
    client.subscribe('subscribe/test')
})
client.on('message', (topic, message) => {
    console.log('%s', message)
})
