const { interval, range, concat } = require('rxjs')
const { take } = require('rxjs/operators')

const a = interval(1000).pipe(take(5))
const b = range(1, 3)
const c = concat(a, b)

c.subscribe(x => console.log(x))
console.log('finish')
