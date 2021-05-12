import { createStore, applyMiddleware } from 'redux'

import createSagaMiddleware from 'redux-saga'
import { debounce, delay } from 'redux-saga/effects'

const reducers = (state, action) => {
    switch (action.type) {
        default:
            return state
    }
}

const handleInput = function*(args, action) {
    if (action.payload === 5) {
        yield delay(5000)
    } else {
        yield delay(1000)
    }
    console.log(args, action)
}

const sagas = function*() {
    yield debounce(500, 'INPUT_CHANGE', handleInput, ['i', 'am', 'hzw'])
}

const sagaMiddleware = createSagaMiddleware()
const store = createStore(reducers, applyMiddleware(sagaMiddleware))
sagaMiddleware.run(sagas)

let i = 0
const tid = setInterval(() => {
    store.dispatch({ type: 'INPUT_CHANGE', payload: i+=1 })
    if (i === 5) {
        clearInterval(tid)
        setTimeout(() => {
            store.dispatch({ type: 'INPUT_CHANGE', payload: 6 })
        }, 501);
    }
}, 499);
