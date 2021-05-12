const fs = require('fs')

/*
//ENOENT
fs.stat('rmdir', (err, stats) => {
    if (err) throw err
    console.log(stats)
})
*/


/*
fs.stat('/var/log/nginx/access.log', (err, stats) => {
    if (err) throw err
    console.log(stats)
})
*/

fs.stat('/var/log/lightdm/lightdm.log', (err, stats) => {
    if (err) throw err
    console.log(stats)
})

/*
fs.stat('rmdir.js', (err, stats) => {
    if (err) throw err
    console.log(stats)
})
*/

console.log('after call fs.')
