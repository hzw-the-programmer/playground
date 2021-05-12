const mqtt = require('mqtt')

if (process.argv.length < 5) {
    console.log('node publish url username password')
    process.exit()
}

const url = process.argv[2]
const username = process.argv[3]
const password = process.argv[4]

const client = mqtt.connect(url, {username, password})

client.on('connect', () => {
    console.log('connected')
})

let count = 0
setInterval(() => {
    client.publish('subscribe/test', `${count++}`)
}, 2000)
