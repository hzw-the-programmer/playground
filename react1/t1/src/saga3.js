import { createStore, applyMiddleware } from 'redux'

import createSagaMiddleware from 'redux-saga'
import { delay, throttle } from 'redux-saga/effects'

const reducers = (state, action) => {
    switch (action.type) {
        default:
            return state
    }
}

const handleChange = function*(args, action) {
    if (action.payload === 0) {
        yield delay(5000)    
    } else {
        yield delay(1000)
    }
    console.log(args, action)
}

const sagas = function*() {
    yield throttle(500, 'INPUT_CHANGE', handleChange, ['i', 'am', 'hzw'])
}

const sagaMiddleware = createSagaMiddleware()
const store = createStore(reducers, applyMiddleware(sagaMiddleware))
sagaMiddleware.run(sagas)

for (let i = 0 ; i < 100; i++) {
    store.dispatch({ type: 'INPUT_CHANGE', payload: i})
}
