function createWS(url) {
    let ws = null
    create()

    return {
        send,
    }

    function create() {
        ws = new WebSocket(url)

        ws.onopen = ev => {
            console.log('onopen', ev)
        }
        
        ws.onmessage = ev => {
            console.log('onmessage', ev)
        }
        
        ws.onclose = ev => {
            console.log('onclose', ev)
            setTimeout(() => {
                create()
            }, 5000)
        }
        
        ws.onerror = ev => {
            console.log('onerror', ev)
        }
    }

    function send(msg) {
        if (!ws) return
        ws.send(msg)
    }
}

createWS('ws://localhost:8000/time')
