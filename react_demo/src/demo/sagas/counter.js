import { delay } from 'redux-saga'
import { call, put, takeEvery } from 'redux-saga/effects'
import { INCREMENT_ASYNC, increment } from "../actions/counter";

function* incrementAsync() {
    yield call(delay, 1000)
    yield put(increment())
}

export default function* watchIncrementAsync() {
    yield takeEvery(INCREMENT_ASYNC, incrementAsync)
}
