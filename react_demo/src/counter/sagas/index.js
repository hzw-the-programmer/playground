import { takeEvery, put } from 'redux-saga/effects'
import { delay } from 'redux-saga'

function* incrementAsync() {
  console.log('incrementAsync before delay')
  let res = yield delay(1000)
  console.log('incrementAsync after delay', res)
  res = yield put({type: 'INCREMENT'})
  console.log('after put', res)
}

export default function* rootSaga() {
  console.log('rootSaga before takeEvery')
  const res = yield takeEvery('INCREMENT_ASYNC', incrementAsync)
  console.log('rootSaga after takeEvery', res)
}