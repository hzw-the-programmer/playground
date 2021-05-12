const fs = require('fs')

/*
fs.readdir('..', (err, files) => {
    console.log(files)
})
*/

/*
fs.readdir('/home/zhiwenhe', (err, files) => {
    console.log(files)
})
*/

/*
fs.readdir('..', {encoding: 'buffer'}, (err, files) => {
    console.log(files)
})
*/

/*
fs.readdir('..', {withFileTypes: true}, (err, files) => {
    files.forEach((value, index, array) => {
        console.log(typeof value, typeof array[index])
    })
})
*/

/*
fs.readdir('notexist', (err, files) => {
    if (err) throw err
})
*/

fs.readdir('/etc/passwd', (err, files) => {
    if (err) throw err
})
