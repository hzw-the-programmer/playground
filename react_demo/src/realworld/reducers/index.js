import { combineReducers } from 'redux'
import _ from 'lodash'
import * as ActionTypes from '../actions'
import paginate from './paginate'

function entities(state = { users: {}, repos: {} }, action) {
  if (action.response && action.response.entities) {
    return _.merge({}, state, action.response.entities)
  }

  return state
}

const pagination = combineReducers({
  starredByUser: paginate({
    mapActionToKey: action => action.login,
    types: [
      ActionTypes.STARRED.REQUEST,
      ActionTypes.STARRED.SUCCESS,
      ActionTypes.STARRED.FAILURE
    ]
  }),
  stargazersByRepo: paginate({
    mapActionToKey: action => action.fullName,
    types: [
      ActionTypes.STARGAZERS.REQUEST,
      ActionTypes.STARGAZERS.SUCCESS,
      ActionTypes.STARGAZERS.FAILURE
    ]
  })
})

const rootReducer = combineReducers({
  entities,
  pagination
})

export default rootReducer
