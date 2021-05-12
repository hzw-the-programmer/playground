let dispatch = function() {
    console.log('haha')
}

const middlewareAPI = {
    dispatch: () => dispatch()
}

middlewareAPI.dispatch()

dispatch = function() {
    console.log('hehe')
}

middlewareAPI.dispatch()