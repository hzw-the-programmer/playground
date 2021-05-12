const http = require('http')

//host = '127.0.0.1'
host = ''
port = 3000

const server = http.createServer((req, res) => {
    res.statusCode = 200
    res.setHeader('Content-Type', 'text/plain')
    res.end('Hello World')
})

server.listen(port, host, () => {
    console.log(`Server running at http://${host}:${port}`)
})
