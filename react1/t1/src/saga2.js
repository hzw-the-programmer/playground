import { createStore, applyMiddleware } from 'redux'
import createSagaMiddleware from 'redux-saga'
import { delay, call, fork, all, take, cancel, put, cancelled } from 'redux-saga/effects'

const reducer = (state, action) => {
}

function* sync() {
    let i = 0
    try {
        while (true) {
            yield delay(1000)
            console.log(`sync${i++}`)
        }
    } finally {
        if (yield cancelled()) {
            console.log('cancelled')
        }
    }
}

function* bgSync() {
    console.log('in bgSync')
    while (yield take('START')) {
        const task = yield fork(sync)
        yield take('CANCEL')
        yield cancel(task)
    }
}

function* ctlSync() {
    console.log('in ctlSync')
    while (true) {
        yield delay(5000)
        yield put({type: 'START'})
        yield delay(5000)
        yield put({type: 'CANCEL'})
    }
}

function* saga() {
    console.log('in saga')
    yield fork(bgSync)
    yield fork(ctlSync)
    console.log('out saga')
}

const sagaMiddleware = createSagaMiddleware()
const store = createStore(reducer, applyMiddleware(sagaMiddleware))
console.log('run(saga)')
sagaMiddleware.run(saga)
console.log('run(saga) finish')
