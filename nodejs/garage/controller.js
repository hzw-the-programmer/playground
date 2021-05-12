const mqtt = require('mqtt')

if (process.argv.length < 5) {
    console.error('url username password')
    process.exit()
}

const url = process.argv[2]
const username = process.argv[3]
const password = process.argv[4]

const client = mqtt.connect(url, {username, password})

let connected = false
let state = ''

client.on('connect', connack => {
    //console.log('connect: ', connack)

    client.subscribe('garage/connected')
    client.subscribe('garage/state')
})

client.on('message', (topic, message) => {
    //console.log('message: ', topic, message)

    switch (topic) {
        case 'garage/connected':
            return handleGarageConnected(message)
        case 'garage/state':
            return handleGarageState(message)
    }
})

/*
client.on('packetsend', packet => {
    console.log('packetsend: ', packet)
})

client.on('packetreceive', packet => {
    console.log('packetreceive: ', packet)
})
*/

function handleGarageConnected(message) {
    connected = (message.toString() === 'true')
    console.log('door connected: %s', message)
}

function handleGarageState(message) {
    state = message
    console.log('door state: %s', message)
}

function openGarageDoor() {
    if (connected && state !== 'open') {
        console.log('open door')
        publish('garage/open', 'true')
    }
}

function closeGarageDoor() {
    if (connected && state !== 'closed') {
        console.log('close door')
        publish('garage/close', 'true')
    }
}

function publish(topic, message) {
    client.publish(topic, message)
}

setTimeout(() => {
    openGarageDoor()
}, 10000)

setTimeout(() => {
    closeGarageDoor()
}, 20000)
