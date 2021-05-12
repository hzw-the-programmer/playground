const fs = require('fs')

fs.rename('/tmp/a', '/tmp/b', err => {
    if (err) throw err
    console.log('success.')
})
