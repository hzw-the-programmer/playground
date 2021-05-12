let a = [1, 2, 3]
let n = a.push(4)
console.log(n, a)

let b = [5, 6]
// n = a.push(b)
n = a.push(...b)
console.log(n, a)
