import { combineReducers } from 'redux'
import {
  INVALIDATE_SUBREDDIT,
  REQUEST_POSTS,
  RECEIVE_POSTS,
  SELECT_SUBREDDIT
} from '../actions'

const posts = (
  state = {
    isFetching: false,
    didInvalidate: false,
    items: []
  }, action
) => {
  switch (action.type) {
    case INVALIDATE_SUBREDDIT:
      return {
        ...state,
        didInvalidate: true
      }
    case REQUEST_POSTS:
      return {
        ...state,
        isFetching: true,
        didInvalidate: false
      }
    case RECEIVE_POSTS:
      return {
        ...state,
        isFetching: false,
        didInvalidate: false,
        items: action.posts,
        lastUpdated: action.receivedAt
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

const selectedSubreddit = (state = 'reactjs', action) => {
  switch (action.type) {
    case SELECT_SUBREDDIT:
      return action.subreddit
    default:
      return state
  }
}

export default combineReducers({
  postsBySubreddit,
  selectedSubreddit
})

// const state = {
//   postsBySubreddit: {
//     frontend: {
//       isFetching: false,
//       didInvalidate: false,
//       items: []
//     },
//     reactjs: {
//       isFetching: false,
//       didInvalidate: false,
//       items: [{
//         id: 1,
//         title: 'Designing Redux State Shape'
//       }, {
//         id: 2,
//         title: 'Designing Reactjs UI Hierarchy'
//       }]
//     }
//   },
//   selectedSubreddit: 'reactjs'
// }
