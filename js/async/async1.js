function a(r) {
    console.log(`a is called with ${r}`)

    return new Promise((resolve, reject) => {
        setTimeout(() => {
            resolve(r)
            console.log('cb:' + r)
        }, 1000)
    })
}

async function b() {
    console.log('b is called')

    console.log(await a(1))
    console.log(await a(2))
    throw new Error(4)
    return 3
}

b().then(value => console.log(`then: ${value}`))
   .catch(err => console.log(`catch: ${err}`))

console.log(0)
