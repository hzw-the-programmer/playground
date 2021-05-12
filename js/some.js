const a = [1, 3, 5, 8, 9, 11]
const r = a.some((e, i, a) => {
    console.log(e, i, a)
    return e % 2 === 0
})

console.log(r)
