const p = new Promise((resolve, reject) => {
    setTimeout(() => {
        resolve('hzw resolved')
    }, 1000)
})

p.then(value => {
    console.log(value)
    p.then(value => console.log(value))
    console.log('after p.then()')
})

console.log('finish')
