const a = [1, 2, 3, 4]

console.log('typeof a:', typeof a)

const b = a

console.log('before change b')
console.log('a', a)
console.log('b', b)

b[0] = 2

console.log('after change b')
console.log('a', a)
console.log('b', b)
