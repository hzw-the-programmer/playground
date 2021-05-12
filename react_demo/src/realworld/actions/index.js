const REQUEST = 'REQUEST'
const SUCCESS = 'SUCCESS'
const FAILURE = 'FAILURE'

function createRequestTypes(base) {
  return [REQUEST, SUCCESS, FAILURE].reduce((acc, type) => {
    acc[type] = `${base}_${type}`
    return acc
  },{})
}

const USER = createRequestTypes('USER')
const REPO = createRequestTypes('REPO')
export const STARRED = createRequestTypes('STARRED')
export const STARGAZERS = createRequestTypes('STARGAZERS')

export const LOAD_USER_PAGE = 'LOAD_USER_PAGE'
export const LOAD_REPO_PAGE = 'LOAD_REPO_PAGE'
export const LOAD_MORE_STARRED = 'LOAD_MORE_STARRED'
export const LOAD_MORE_STARGAZERS = 'LOAD_MORE_STARGAZERS'

function action(type, payload) {
  return {type, ...payload}
}

export const user = {
  request: login => action(USER[REQUEST], { login }),
  success: (login, response) => action(USER[SUCCESS], { login, response }),
  failure: (login, error) => action(USER[FAILURE], { login, error })
}
export const repo = {
  request: fullName => action(REPO[REQUEST], { fullName }),
  success: (fullName, response) => action(REPO[SUCCESS], { fullName, response }),
  failure: (fullName, error) => action(REPO[FAILURE], { fullName, error })
}
export const starred = {
  request: login => action(STARRED[REQUEST], { login }),
  success: (login, response) => action(STARRED[SUCCESS], { login, response }),
  failure: (login, error) => action(STARRED[FAILURE], { login, error })
}
export const stargazers = {
  request: fullName => action(STARGAZERS[REQUEST], { fullName }),
  success: (fullName, response) => action(STARGAZERS[SUCCESS], { fullName, response }),
  failure: (fullName, error) => action(STARGAZERS[FAILURE], { fullName, error })
}

export const loadUserPage = login => action(LOAD_USER_PAGE, { login })
export const loadRepoPage = fullName => action(LOAD_REPO_PAGE, { fullName })
export const loadMoreStarred = login => action(LOAD_MORE_STARRED, { login })
export const loadMoreStargazers = fullName => action(LOAD_MORE_STARGAZERS, { fullName })
