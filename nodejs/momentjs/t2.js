const moment = require('moment')
const ts = moment('2019-11-07 07:20:38').unix()
console.log(moment.unix(ts).format('YYYY-MM-DD HH:mm:ss'))
