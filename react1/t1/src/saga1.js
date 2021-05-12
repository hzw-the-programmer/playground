import { createStore, applyMiddleware } from 'redux'
import createSagaMiddleware from 'redux-saga'
import { delay, call, fork, all } from 'redux-saga/effects'

const reducer = (state, action) => {
    console.log('reducers', state, action)
}

function* fetchResource(res) {
    console.log('fetchResource', res, 'begin')
    yield delay(5000)
    console.log('fetchResource', res, 'end')
}

function* fetchAll() {
    // console.log('1')
    // const task1 = yield fork(fetchResource, 'users')
    // console.log('2')
    // const task2 = yield fork(fetchResource, 'comments')
    // console.log('3')
    // return 'ok'
    console.log(yield all([
        fork(fetchResource, 'users'),
        fork(fetchResource, 'comments'),
    ]))
    return 'ok'
}

function* saga() {
    console.log('in saga')
    console.log(yield call(fetchAll))
    console.log('out saga')
}

const sagaMiddleware = createSagaMiddleware()
const store = createStore(reducer, applyMiddleware(sagaMiddleware))
console.log('before saga')
sagaMiddleware.run(saga)
console.log('after saga')
