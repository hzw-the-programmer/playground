function* gen1() {
    throw new Error('g1 err')
}

function* gen2() {
    console.log('g2', yield 1)
    // try {
        console.log('g2', yield* gen1())
    // } catch (err) {
    // }
    throw new Error('g2 err')
    console.log('g2', yield 2)
}

const g = gen2()
console.log(g.next())
try {
    console.log(g.next('f1'))
} catch (err) {
    console.log('catched')
    console.log(err)
}
