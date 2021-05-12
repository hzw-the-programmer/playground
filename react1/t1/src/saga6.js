import { createStore, applyMiddleware } from 'redux'
import createSagaMiddleware from 'redux-saga'
import { delay, put, cancelled, takeLatest } from 'redux-saga/effects'


const reducers = (state, action) => {
    console.log('reducers', state, action)
    switch (action.type) {
        case 'savePerson':
            return action.payload
        default:
            return state
    }
}

const save = function*(args, action) {
    console.log('save', args, action)
    try {
        yield delay(1000)
        yield put({ type: 'savePerson', payload: 1})
        console.log('after put')
    } finally {
        if (yield cancelled()) {
            console.log('cancelled')
        }
    }
}

const sagas = function*() {
    yield takeLatest('queryPerson', save, ['i', 'am', 'hzw'])
}

const sagaMiddleware = createSagaMiddleware()
const store = createStore(reducers, applyMiddleware(sagaMiddleware))
sagaMiddleware.run(sagas)

store.subscribe(() => console.log('ui', store.getState()))

store.dispatch({type: 'queryPerson'})
store.dispatch({type: 'queryPerson'})
