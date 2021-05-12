
//const p = Promise.all([])
//const p = Promise.all([Promise.resolve(1), Promise.resolve(2)])
const p = Promise.all([1, 2])
//const p = Promise.all([1, Promise.resolve(2)])
/*
const p1 = Promise.resolve(1)
const p2 = Promise.resolve(2)
const p3 = new Promise((resolve, reject) => {
    setTimeout(resolve, 1000, 'zhiwenhe')
})
const p = Promise.all([p1, p2, p3])
*/
console.log(p)
console.log(1)
p.then(value => {
    console.log(value)
})
console.log(2)
