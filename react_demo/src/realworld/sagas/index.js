import { all, call, fork, put, select, take } from 'redux-saga/effects'
import { api } from '../services'
import * as actions from '../actions'
import { getUser, getRepo, getStarredByUser, getStargazersByRepo } from '../reducers/selectors'

const { user, repo, starred, stargazers } = actions

const firstPageStarredUrl = login => `users/${login}/starred`
const firstPageStargazersUrl = fullName => `repos/${fullName}/stargazers`

function* fetchEntity(entity, apiFn, id, url) {
  yield put( entity.request(id) )
  const { response, error } = yield call(apiFn, url || id)
  if (response)
    yield put( entity.success(id, response) )
  else
    yield put( entity.failure(id, error) )
}

const fetchUser = fetchEntity.bind(null, user, api.fetchUser)
const fetchRepo = fetchEntity.bind(null, repo, api.fetchRepo)
const fetchStarred = fetchEntity.bind(null, starred, api.fetchStarred)
const fetchStargazers = fetchEntity.bind(null, stargazers, api.fetchStargazers)

function* loadUser(login) {
  const user = yield select(getUser, login)
  if (!user) {
    yield call(fetchUser, login)
  }
}

function* loadRepo(fullName) {
  const repo = yield select(getRepo, fullName)

  if (!repo) {
    yield call(fetchRepo, fullName)
  }
}

function* loadStarred(login, loadMore) {
  const starredByUser = yield select(getStarredByUser, login)
  if (!starredByUser || !starredByUser.pageCount || loadMore) {
    yield call(
      fetchStarred,
      login,
      (starredByUser && starredByUser.nextPageUrl) || firstPageStarredUrl(login)
    )
  }
}

function* loadStargazers(fullName, loadMore) {
  const stargazersByRepo = yield select(getStargazersByRepo, fullName)
  if (!stargazersByRepo || !stargazersByRepo.pageCount || loadMore) {
    yield call(
      fetchStargazers,
      fullName,
      (stargazersByRepo && stargazersByRepo.nextPageUrl) || firstPageStargazersUrl(fullName)
    )
  }
}

function* watchLoadUserPage() {
  while (true) {
    const { login } = yield take(actions.LOAD_USER_PAGE)

    yield fork(loadUser, login)
    yield fork(loadStarred, login)
  }
}

function* watchLoadRepoPage() {
  while (true) {
    const { fullName } = yield take(actions.LOAD_REPO_PAGE)

    yield fork(loadRepo, fullName)
    yield fork(loadStargazers, fullName)
  }
}

function* watchLoadMoreStarred() {
  while (true) {
    const { login } = yield take(actions.LOAD_MORE_STARRED)
    yield fork(loadStarred, login, true)
  }
}

function* watchLoadMoreStargazers() {
  while (true) {
    const { fullName } = yield take(actions.LOAD_MORE_STARGAZERS)
    yield fork(loadStargazers, fullName, true)
  }
}

export default function* rootSaga() {
  yield all([
    fork(watchLoadUserPage),
    fork(watchLoadRepoPage),
    fork(watchLoadMoreStarred),
    fork(watchLoadMoreStargazers)
  ])
}
