const a = ['1', '2', '3', '4', '5']
const reducer = (accumulator, e) => accumulator + e

let r = a.reduce(reducer)
console.log(r)

r = a.reduce(reducer, 5)
console.log(r)
