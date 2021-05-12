function downloadData(url) {
    return new Promise((resolve, reject) => {
        console.log("== download data begin ==")
        setTimeout(() => {
            console.log("== download data successed ==")
            resolve("downloaded data")
            // console.log("== download data failed ==")
            // reject("downloaded data error")
        }, 1000)
    })
}

function downloadFallbackData(url) {
    return new Promise(resolve => {
        console.log("== download fallback data begin ==")
        setTimeout(() => {
            console.log("== download fallback data successed ==")
            resolve("downloaded fallback data")
            // console.log("== download fallback data failed ==")
            // reject("downloaded fallback data error")
        }, 1000)
    })
}

function processDataInWorker(data) {
    return new Promise(resolve => {
        console.log("== process data begin ==")
        setTimeout(() => {
            console.log("== process data successed ==")
            resolve(`${data} processed`)
        }, 1000)
    })
}

function* getProcessedData(url) {
    let downloadedData
    try {
        downloadedData = yield downloadData(url)
    } catch (err) {
        downloadedData = yield downloadFallbackData(url)
    }
    return yield processDataInWorker(downloadedData)
}

const g = getProcessedData("github.com")
g.next().value
    .then(
        downloadedData => g.next(downloadedData).value,
        err => g.throw(err).value.then(downloadedFallbackData => g.next(downloadedFallbackData).value)
    )
    .then(processedData => console.log(g.next(processedData).value))

console.log("end")
