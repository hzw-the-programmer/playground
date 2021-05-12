import { all, call, fork, put, select, take } from 'redux-saga/effects'
import { api } from '../services'
import * as actions from '../actions'
import { getPlace } from '../reducers/selectors'

function* fetchEntity(entity, apiFn, id, url) {
  yield put(entity.request(id))
  const {response, error} = yield call(apiFn, url || id)
  if (response)
    yield put(entity.success(id, response))
  else
    yield put(entity.failure(id, error))
}

const fetchPlace = fetchEntity.bind(null, actions.place, api.fetchPlace)

function* loadPlace(id) {
  const place = yield select(getPlace, id)
  if (!place) {
    yield call(fetchPlace, id)
  }
}

function* watchLoadPlace() {
  while (true) {
    const { id } = yield take(actions.LOAD_PLACE)
    yield fork(loadPlace, id)
  }
}

function* rootSaga() {
  yield all([
    fork(watchLoadPlace)
  ])
}

export default rootSaga
