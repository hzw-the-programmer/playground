const fs = require('fs')

/*
fs.readFile('/etc/passwd', (err, data) => {
    console.log(data)
})
*/

/*
fs.readFile('/etc/passwd', 'utf8', (err, data) => {
    console.log(data)
})
*/

/*
fs.readFile('/etc/passwd', {encoding: 'utf8'}, (err, data) => {
    console.log(data)
})
*/

/*
fs.readFile('/etc/passwd', {}, (err, data) => {
    console.log(data)
})
*/

/*
fs.readFile('/var/log/nginx/access.log', (err, data) => {
    if (err) throw err
    console.log(data)
})
*/

//EACCES
fs.readFile('/var/log/lightdm/lightdm.log', (err, data) => {
    if (err) throw err
    console.log(data)
})

console.log('after fs call.')
