const fs = require('fs')

/*
//EEXIST
fs.symlink('stat.js', '/tmp/symlink', err => {
    if (err) throw err
    console.log('success.')
})
*/

fs.realpath('stat.js', (err, resolvedPath) => {
    if (err) throw err
    console.log(resolvedPath)
    fs.symlink(resolvedPath, '/tmp/symlink', err => {
        if (err) throw err
        console.log('success.')
    })
})

console.log('after fs.symlink call.')
