function* gen1() {
    console.log(yield* [1, 2])
    // console.log(yield* 'ab')
}

const g = gen1()
console.log(g.next())
console.log(g.next())
console.log(g.next())
