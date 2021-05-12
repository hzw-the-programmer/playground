function once(cb) {
    let called = false
    return function() {
        if (!called) {
            called = true
            cb()
        }
    }
}

let func = once(function() {
    console.log('func1 called');
})

console.log('first call')
func()
console.log('second call')
func()

func = once(function() {
    console.log('func2 called');
})

console.log('first call')
func()
console.log('second call')
func()
