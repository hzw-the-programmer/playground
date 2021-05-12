import { FETCH_USER_REQUESTED, FETCH_USER_SUCCEEDED, FETCH_USER_FAILED } from '../actions'

const root = (state, action) => {
  switch (action.type) {
    case FETCH_USER_REQUESTED:
      return {loading: true}
    case FETCH_USER_SUCCEEDED:
      return {loading: false, user: action.user}
    case FETCH_USER_FAILED:
      return {loading: false, error: action.error}
    default:
      return state
  }
}

export default root