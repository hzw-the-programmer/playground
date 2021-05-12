const express = require('express')

const app = express()

// curl http://localhost:8000
app.get('/', function(req, res) {
    res.end('get /');
})

// curl --data "name=zhiwenhe" http://localhost:8000
app.post('/', function(req, res) {
    res.end('post /');
})

app.listen(8000);
