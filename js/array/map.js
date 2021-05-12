let a = [1, 2, 3]

let b = [1, 2, 3]
console.log('a == b ?', a == b)

b = a
console.log('a == b ?', a == b)

b = a.map(e => e)
console.log('a == b ?', a == b)

const e1 = {key: 'value'}
const e2 = {}
console.log('e1 == e2 ?', e1 == e2)

a = [{}, e1, {}, e2, {}, {}]
b = a.map(e => {
    if (e == e1)
        return {}
    else
        return e
})

a.forEach((c, i, a) => {
    if (c != b[i]) {
        console.log(c)
    }
})
