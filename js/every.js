const a = [1, 30, 39, 41, 29, 10, 13]
const r = a.every((e, i, a) => {
    console.log(e, i, a)
    return e < 40
})
console.log(r, a)
