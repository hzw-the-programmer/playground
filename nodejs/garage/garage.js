const mqtt = require('mqtt')

if (process.argv.length < 5) {
    console.error('url username password')
    process.exit()
}

const url = process.argv[2]
const username = process.argv[3]
const password = process.argv[4]

const client = mqtt.connect(url, {username, password})

let state = 'closed'

client.on('connect', connack => {
    //console.log('connect: ', connack)
    
    client.subscribe('garage/open')
    client.subscribe('garage/close')

    publish('garage/connected', 'true')
    sendStateUpdate()
})

client.on('message', (topic, message) => {
    //console.log('message: ', topic, message)

    switch (topic) {
        case 'garage/open':
            return handleOpenRequest()
        case 'garage/close':
            return handleCloseRequest()
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

function sendStateUpdate() {
    publish('garage/state', state)
}

function handleOpenRequest() {
    if (state !== 'open' && state !== 'opening') {
        state = 'opening'
        sendStateUpdate()
        console.log(state)

        setTimeout(() => {
            state = 'open'
            sendStateUpdate()
            console.log(state)
        }, 5000)
    }
}

function handleCloseRequest() {
    if (state !== 'closed' && state !== 'closing') {
        state = 'closing'
        sendStateUpdate()
        console.log(state)

        setTimeout(() => {
            state = 'closed'
            sendStateUpdate()
            console.log(state)
        }, 5000)
    }
}

function publish(topic, message) {
    client.publish(topic, message/*, {retain: true}*/)
}

process.on('SIGINT', function(signal) {
    console.log(`signal: ${signal}`)
    publish('garage/connected', 'false')
    setTimeout(() => {
        process.exit()
    }, 5000)
})
