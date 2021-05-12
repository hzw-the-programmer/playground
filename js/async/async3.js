function resolveAfter2Seconds() {
    console.log("slow resolve begin")
    return new Promise(resolve => {
        setTimeout(() => {
            resolve("slow")
            console.log("slow resolve done")
        }, 2000)
    })
}

function resolveAfter1Second() {
    console.log("fast resolve begin")
    return new Promise(resolve => {
        setTimeout(() => {
            resolve("fast")
            console.log("fast resolve done")
        }, 1000)
    })
}

async function sequentialStart() {
    console.log("== SEQUENTIAL START ==")
    let slow = await resolveAfter2Seconds()
    console.log(slow)
    let fast = await resolveAfter1Second()
    console.log(fast)
}

async function concurrentStart() {
    console.log("== CONCURRENT START ==")
    const slow = resolveAfter2Seconds()
    const fast = resolveAfter1Second()
    console.log(await slow)
    console.log(await fast)
    // 1 + 1
    return 1 + 1
}

function concurrentPromise() {
    console.log("== CONCURRENT PROMISE ==")
    Promise.all([resolveAfter2Seconds(), resolveAfter1Second()]).then(messages => {
        console.log(messages[0])
        console.log(messages[1])
    })
}

function parallel() {
    console.log("== PARALLEL ==")
    Promise.all([
        (async() => console.log(await resolveAfter2Seconds()))(),
        (async() => console.log(await resolveAfter1Second()))()
    ])
}

function parallelPromise() {
    console.log("== PARALLEL PROMISE ==")
    resolveAfter2Seconds().then(msg => console.log(msg))
    resolveAfter1Second().then(msg => console.log(msg))
}

// sequentialStart().then(value => console.log(`then: ${value}`))
concurrentStart().then(value => console.log(`then: ${value}`))
// concurrentPromise()
// parallel()
// parallelPromise()

console.log("end")
