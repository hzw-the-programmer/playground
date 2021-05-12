const a = ['a', 'b', 'c']
const b = [1, 2, 3]

a.push(1)
console.log(a)
a.push(2, 3)
console.log(a)
a.push(b)
console.log(a)
a.push.apply(a, b)
console.log(a)
