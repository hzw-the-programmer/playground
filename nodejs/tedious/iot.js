const Connection = require('tedious').Connection

const config = {
    server: '10.0.2.2',
    userName: 'sa',
    password: '123456',    
    options: {
        database: 'iot',
        encrypt: true,
        debug: {
            packet: true,
            data: true,
            payload: true,
            token: false,
            log: true
        }
    }
}

const conn = new Connection(config)

conn.on('connect', err => {
    if (err) {
        console.log('conn err', err)
    } else {
        executeStatement()
    }
})

conn.on('debug', text => {
    console.log(text)
})

function executeStatement() {
    const Request = require('tedious').Request

    const req = new Request('SELECT 1, 2, 3, 4, NULL', (err, count) => {
        if (err) {
            console.log('req err', err)
        } else {
            console.log(`${count} rows`)
        }

        conn.close()
    })

    req.on('row', columns => {
        columns.forEach(column => {
            console.log(column.value)
        })
    })

    req.on('done', (count, more) => {
        console.log('done', count, more)
    })

    conn.execSql(req)
}
