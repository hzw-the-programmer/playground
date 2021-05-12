console.log(`worker ${process.pid} started`)

setTimeout(() => {
    console.log(`worker ${process.pid} timeout`)
    setTimeout(() => {
        process.send(`msg from worker ${process.pid}`)
    }, 2000)
    
}, 2000)

process.on('message', msg => {
    console.log('message:', msg)

    process.exit(11)
})
