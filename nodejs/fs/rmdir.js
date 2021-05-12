const fs = require('fs')

/*
//ENOENT
fs.rmdir('/tmp/notexist', err => {
    if (err) throw err
    console.log('success.')
})
*/

/*
//ENOTDIR
fs.rmdir('/tmp/file', err => {
    if (err) throw err
    console.log('success.')
})
*/

//ENOTEMPTY
fs.rmdir('/tmp/dir', err => {
    if (err) throw err
    console.log('success.')
})

console.log('after async fs call.')
