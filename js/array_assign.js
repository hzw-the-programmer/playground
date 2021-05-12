/*
a = [1, 2, 3]
b = a
b[0] = 0
console.log(a)
console.log(b)
*/

/*
a = [1, 2, 3]
b = [...a]
b[0] = 0
console.log(a)
console.log(b)
*/

a = [1, 2, 3]
b = a.slice()
b[0] = 0
console.log(a)
console.log(b)
