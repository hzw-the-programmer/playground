/*
new Promise((resolve, reject) => {
    reject('something wrong')
})
*/

try {
    new Promise((resolve, reject) => {
        reject('something wrong')
    })
} catch (error) {
    console.log(error)
}
