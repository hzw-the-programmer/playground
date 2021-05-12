function* gen1() {

}

function* gen2() {
    // try {
        console.log('g2', yield 1)
    // } catch (err) {
        console.log(err)
    // }
}

const g = gen2()
console.log(g.next())
try {
    console.log(g.throw(new Error('haha')))
} catch (err) {
   console.log(err)
}
console.log('eeee')
