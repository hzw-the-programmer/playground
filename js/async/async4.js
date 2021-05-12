function downloadData(url) {
    return new Promise((resolve, reject) => {
        console.log("== download data begin ==")
        setTimeout(() => {
            // console.log("== download data successed ==")
            // resolve("downloaded data")
            console.log("== download data failed ==")
            reject("download data error")
        }, 2000)
    })
}

function downloadFallbackData(url) {
    return new Promise(resolve => {
        console.log("== download fallback data begin ==")
        setTimeout(() => {
            console.log("== download fallback data successed ==")
            resolve("downloaded fallback data")
        }, 2000)
    })
}

function processDataInWorker(data) {
    return new Promise(resolve => {
        console.log("== process data begin ==")
        setTimeout(() => {
            console.log("== process data end ==")
            resolve(`processed ${data}`)
        }, 2000)
    })
}

async function getProcessedData(url) {
// function getProcessedData(url) {
    // return downloadData(url)
    //     .then(data => processDataInWorker(data))
    //     .catch(e => downloadFallbackData(url))

    // return downloadData(url)
    //     .catch(e => downloadFallbackData(url))
    //     .then(data => processDataInWorker(data))

    let data
    try {
        data = await downloadData(url)
    } catch {
        data = await downloadFallbackData(url)
    }

    return processDataInWorker(data)
}

getProcessedData("github.com").then(data => console.log(data))
