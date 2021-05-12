const p1 = new Promise((resolve, reject) => {
    setTimeout(() => {
        console.log('p1 resolve')
        resolve('p1')
    }, 2000)
})

const p2 = new Promise((resolve, reject) => {
    setTimeout(() => {
        console.log('p2 reject')
        reject('p2')
    }, 1000)
})

Promise.all([p1, p2]).then(values => console.log(values)).catch(e => console.log(e))
