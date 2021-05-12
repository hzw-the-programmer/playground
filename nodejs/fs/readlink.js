const fs = require('fs')

fs.readlink('/home/zhiwenhe/bin/node', (err, linkString) => {
    if (err) throw err
    console.log(linkString)
})
