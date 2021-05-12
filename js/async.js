async function func1() {
    console.log('in func1')
    /*
    promise = new Promise(resolve => {
        console.log('constructing promise')
        setTimeout(resolve, 5000)
    })
    */
    promise = Promise.resolve(33)
    console.log('before await')
    await promise
    console.log('after await')
}

console.log('begin call func1')
func1().then(value => console.log('haha', value))
console.log('after call func1')
