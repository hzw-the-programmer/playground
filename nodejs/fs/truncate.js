const fs = require('fs')

/*
fs.truncate('/tmp/abc', err => {
    if (err) throw err
    console.log('success.')
})
*/

/*
fs.truncate('/tmp/abc', 2, err => {
    if (err) throw err
    console.log('success.')
})
*/

//ENOENT
fs.truncate('/tmp/notexist', 2, err => {
    if (err) throw err
    console.log('success.')
})

console.log('after call fs.truncate.')
