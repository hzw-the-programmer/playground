new Promise((resolve, reject) => {
    console.log('1')
    setTimeout(() => {
        console.log('timeout brefore resolve')
        //resolve('r1')
        reject('e1')
        console.log('timeout after resolve')
    }, 1000)
}).then(value => {
    console.log('2', value)
}/*, error => {
    console.log('2', error)
}*/).catch(error => {
    console.log('3', error)
}).then(value => {
    console.log('4', value)
})
