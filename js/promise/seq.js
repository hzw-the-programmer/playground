function startWork(name, resolve, reject) {
    setTimeout(() => {
        const r = Math.floor(Math.random() * 100)
        console.log(name, r)
        r > 50 ? resolve(`${name} resolved`) : reject(`${name} rejected`)
    }, 1000)
}

function doSomething() {
    return new Promise((resolve, reject) => {
        startWork('something', resolve, reject)
    })
}

function doSomethingElse() {
    return new Promise((resolve, reject) => {
        startWork('something else', resolve, reject)
    })
}

function doThirdSomething() {
    return new Promise((resolve, reject) => {
        startWork('third thing', resolve, reject)
    })
}

doSomething()
.then(result => doSomethingElse())
.then(result => doThirdSomething())
.then(result => console.log(result))
.catch(error => console.log(error))
