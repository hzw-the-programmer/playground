a = [1, 2, 3]
b = [1, 2, 3]
console.log('a == b', a == b)
console.log('a === b', a === b)

console.log(a.every((e, i) => e === b[i]))

b = a
console.log('a == b', a == b)
console.log('a === b', a === b)

console.log(typeof a)
console.log(Object.prototype.toString.call(a))
console.log(a + '')
console.log(a.toString())

console.log(a.toString.toString())
console.log(Object.prototype.toString.toString())
console.log(a.toString === Object.prototype.toString)

c = {}
console.log(c.toString === Object.prototype.toString)

console.log(a.toString.toString === c.toString.toString)
