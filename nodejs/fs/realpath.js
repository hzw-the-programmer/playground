const fs = require('fs')

fs.realpath('/home/zhiwenhe/bin/node', (err, resolvedPath) => {
    if (err) throw err
    console.log(resolvedPath)
})
