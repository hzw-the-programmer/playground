import { all, call, fork, put, takeLatest } from 'redux-saga/effects'

import { api } from '../services'
import {
  SUBMIT_PRICE_FORM,
  submitPriceFormRequest,
  submitPriceFormSuccess,
  submitPriceFormFailure
} from '../actions'

import watchIncrementAsync from './counter'

function* submitPriceForm(action) {
  console.log(action)
  yield put(submitPriceFormRequest())

  const { response, error } = yield call(api.submitPriceForm, action.values)
  console.log(response, error)

  if (response)
    yield put(submitPriceFormSuccess({response}))
  else
    yield put(submitPriceFormFailure({error}))
}

function* watchSubmitPriceForm() {
  yield takeLatest(SUBMIT_PRICE_FORM, submitPriceForm)
}

export default function* rootSaga() {
  yield all([
    fork(watchSubmitPriceForm),
    fork(watchIncrementAsync),
  ])
}
