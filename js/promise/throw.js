new Promise((resolve, reject) => {
    throw new Error('hzw')
}).then(value => {
    console.log('resolved', value)
}).catch(error => {
    console.log('rejected', error.message)
})

console.log('finish')
