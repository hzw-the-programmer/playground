const { interval } = require('rxjs')

const numbers = interval(1000)

numbers.subscribe(num => console.log((new Date()).toString(), num))
console.log('finish')
