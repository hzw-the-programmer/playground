function compose(...funcs) {
    return funcs.reduce((a, b) => (...args) => a(b(...args)))
}

function a(...args) {
    console.log('a', args)
    return 'a'
}

function b(...args) {
    console.log('b', args)
    return 'b'
}

function c(...args) {
    console.log('c', args)
    return 'c'
}

funcs = [a, b, c]

console.log(compose(...funcs)('d'))
