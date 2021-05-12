const fs = require('fs')

/*
//ENOSPC
fs.watch('/tmp', (event, filename) => {
    console.log(`event: ${event}, filename: ${filename}`)
})
*/

fs.watch('.', (event, filename) => {
    console.log(`event: ${event}, filename: ${filename}`)
})
