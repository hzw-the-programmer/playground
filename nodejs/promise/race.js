const p1 = new Promise((resolve, reject) => {
    setTimeout(() => {
        console.log('p1 resolve')
        resolve('p1')
    }, 1000)
})

const p2 = new Promise((resolve, reject) => {
    setTimeout(() => {
        console.log('p2 resolve')
        resolve('p2')
    }, 2000)
})

Promise.race([p1, p2]).then(v => console.log(v))
