import { takeEvery, takeLatest, delay, put } from 'redux-saga/effects'

function* incrementAsync() {
  yield delay(1000)
  yield put({type: 'INCREMENT'})
}

export default function* rootSsaga() {
  //yield takeEvery('INCREMENT_ASYNC', incrementAsync)
  yield takeLatest('INCREMENT_ASYNC', incrementAsync)
}