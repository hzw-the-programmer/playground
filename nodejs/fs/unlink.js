const fs = require('fs')

fs.unlink('/tmp/abc', err => {
    if (err) throw err
    console.log('success.')
})

console.log('after call fs.unlink.')
