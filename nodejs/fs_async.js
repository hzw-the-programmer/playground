const fs = require('fs')

console.log('begin readFile call')
fs.readFile('hello.js', (err, data) => {
    if (err) throw err
    console.log(data)
})
console.log('end readFile call')
