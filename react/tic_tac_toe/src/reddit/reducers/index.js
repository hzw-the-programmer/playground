import { combineReducers } from 'redux'
import { SELECT_SUBREDDIT,
         INVALIDATE_SUBREDDIT,
         REQUEST_POSTS,
         RECEIVE_POSTS } from '../actions'

const selectedSubreddit = (state = 'reactjs', action) => {
  switch (action.type) {
    case SELECT_SUBREDDIT:
      return action.subreddit
    default:
      return state
  }
}

const posts = (state = {
  posts: [],
  isFetching: false,
  didInvalidate: false
}, action) => {
  switch (action.type) {
    case INVALIDATE_SUBREDDIT:
      return {...state, didInvalidate: true}
    case REQUEST_POSTS:
      return {...state, isFetching: true, didInvalidate: false}
    case RECEIVE_POSTS:
      return {
        posts: action.posts,
        isFetching: false,
        didInvalidate: false,
        lastUpdated: action.lastUpdated
      }
    default:
      return state
  }
}

const postsBySubreddit = (state = {}, action) => {
  switch (action.type) {
    case INVALIDATE_SUBREDDIT:
    case REQUEST_POSTS:
    case RECEIVE_POSTS:
      return {...state, [action.subreddit]: posts(state[action.subreddit], action)}
    default:
      return state
  }
}

const rootReducer = combineReducers({
  selectedSubreddit,
  postsBySubreddit
})

export default rootReducer
