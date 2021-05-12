function* gen1() {
    console.log('gen1', yield 'a')
    console.log('gen1', yield 'b')
    console.log('gen1', yield 'c')
    // return 'h'
}

function* gen2() {
    console.log('gen2', yield 1)
    console.log('gen2', yield* gen1())
    console.log('gen2', yield 2)
}

const g = gen2()
console.log(g.next())
console.log(g.next('f1'))
console.log(g.next('f2'))
console.log(g.next('f3'))
console.log(g.next('f4'))
console.log(g.next('f5'))
