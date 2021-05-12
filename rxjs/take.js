const { interval } = require('rxjs')
const { take } = require('rxjs/operators')

const numbers = interval(1000).pipe(take(10))
numbers.subscribe(num => console.log((new Date()).toString(), num))
console.log('finish')
