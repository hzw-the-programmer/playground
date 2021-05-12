// new Promise((resolve, reject) => {
//   console.log(1)
//   resolve('resolve')
// }).then(value => {
//   console.log(3 + ' ' + value)
//   return 'then' // return a value
// }).then(value => {
//   console.log(4 + ' ' + value)
//   throw new Error('error') // throw a error
// }).then(value => {
//   console.log(value)
// }, error => {
//   console.log(5 + ' ' + error)
//   return new Promise((resolve, reject) => { // return a Promise and resolve
//     resolve('return a resolved Promise')
//   })
// }).then(value => {
//   console.log(6 + ' ' + value)
//   return new Promise((resolve, reject) => { // return a Promise and reject
//     reject('return a rejected Promise')
//   })
// }).then(value => {
//   console.log(value)
// }, error => {
//   console.log(7 + ' ' + error)
// })
// console.log(2)

new Promise((resolve, reject) => {
  reject('reject')
}).then(value => value, error => error)
  .then(value => {
    console.log('resolve ' + value)
    throw 'error'
  }, error => console.log(error))
  .then(value => {
    console.log('resolve ' + value)
  }, error => {
    console.log('reject ' + error)
    // return 'hah'
  })
  .then(value => {
    console.log(value)
    throw new Error('Zhiwen He')
  })
  .then(value => value)
  .catch(error => console.log(error))