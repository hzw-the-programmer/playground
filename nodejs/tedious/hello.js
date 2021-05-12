const Connection = require('tedious').Connection

const config = {
    userName: 'sa',
    password: '123456',
    server: '10.0.2.2'
}

const conn = new Connection(config)

conn.on('connect', err => {
    if (err) {
        console.log('conn err', err)
        return
    }

    const Request = require('tedious').Request

    const req = new Request("SELECT 'hzw', 23", (err, count) => {
        if (err) {
            console.log('req err', err)
        } else {
            console.log(`${count} rows`)
        }
    })

    req.on('row', columns => {
        columns.forEach(column => {
            console.log(column.value)
        })
    })

    conn.execSql(req)
})
