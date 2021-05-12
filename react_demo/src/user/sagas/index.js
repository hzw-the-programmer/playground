import { all, call, fork, put, select, takeEvery } from 'redux-saga/effects'
import {
  LOAD_USER,
  FETCH_USER_REQUESTED,
  FETCH_USER_SUCCEEDED,
  FETCH_USER_FAILED } from '../actions'
import * as api from '../services'

// function* log(action) {
//   const state = yield select()
//   console.log(action)
//   console.log(state)
// }

// function* watchAndLog() {
//   yield takeEvery('*', log)
// }

function* fetchUser(id) {
  try {
    yield put({type: FETCH_USER_REQUESTED, id})
    const user = yield call(api.fetchUser, id)
    yield put({type: FETCH_USER_SUCCEEDED, user})
  } catch (error) {
    yield put({type: FETCH_USER_FAILED, error})
  }
}

function* loadUser(action) {
  yield call(fetchUser, action.id)
}

function* watchLoadUser() {
  yield takeEvery(LOAD_USER, loadUser)
}

function* root() {
  yield all([
    // fork(watchAndLog),
    fork(watchLoadUser)
  ])
}

export default root