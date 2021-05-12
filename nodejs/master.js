console.log(`master ${process.pid} started`)

const cp = require('child_process')

const c = cp.fork('worker.js')

c.on('message', msg => {
    console.log('message:', msg)
    setTimeout(() => {
        c.send(`msg from master ${process.pid}`)
    }, 2000)
})

c.on('exit', (code, signal) => {
    console.log('exit:', code, signal)
})

process.on('message', msg => {
    console.log('message:', msg)
})
