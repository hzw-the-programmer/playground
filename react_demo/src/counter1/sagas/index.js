import { delay } from 'redux-saga'
import { all, fork, put, takeEvery, takeLatest } from 'redux-saga/effects'
import { INCREMENT, INCREMENT_ASYNC } from '../actions'

function* incrementAsync(action) {
  // console.log('saga', action)
  yield delay(1000)
  yield put({type: INCREMENT})
}

function* watchIncrementAsync() {
  yield takeEvery(INCREMENT_ASYNC, incrementAsync)
  // yield takeLatest(INCREMENT_ASYNC, incrementAsync)
}

function* root() {
  yield all([
    fork(watchIncrementAsync)
  ])
}

export default root