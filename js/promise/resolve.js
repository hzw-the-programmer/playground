Promise.resolve('hzw resolved!').then(value => console.log(value))
Promise.reject('hzw rejected!').catch(reason => console.log(reason))
console.log('finish')
