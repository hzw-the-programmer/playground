/**
 * Author: Zhiwen He <18676797056@163.com>
 */
import { call, fork, put } from 'redux-saga/effects'

import * as api from '../services'

function* initializeBasicTable() {
  const { response, error } = yield call(api.fetchBasicTable)
  if (response)
    yield put({type: 'BASIC_TABLE_FETCHED', payload: response})
}

export default function* rootSaga() {
  yield fork(initializeBasicTable)
}
