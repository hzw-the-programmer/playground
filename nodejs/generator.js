const foo = (name, callback) => {
    setTimeout(() => {
        callback(name)
    }, 1000)
}

foo('a',
    a => foo('b',
        b => foo('c',
            c => console.log(a, b, c)
        )
    )
)

const curry = (func, ...args) => {
    return callback => {
        args.push(callback)
        func.apply({}, args)
    }
}

curry(foo, 'a')(
    a => curry(foo, 'b')(
        b => curry(foo, 'c')(
            c => console.log(a, b, c)
        )
    )
)

const controller = generator => {
    const iterator = generator()

    const advancer = response => {
        const state = iterator.next(response)
        if (!state.done) {
            state.value(advancer)
        }
    }

    advancer()
}

controller(function* () {
    const a = yield curry(foo, 'a')
    const b = yield curry(foo, 'b')
    const c = yield curry(foo, 'c')
    console.log(a, b, c)
})
