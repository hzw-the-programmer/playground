import { combineReducers } from 'redux'
import _ from 'lodash'
import * as ActionTypes from '../actions'

const entities = (state = {
  places: {1: {id: 1, name: '根', pid: 0, level: 0}}
}, action) => {
  if (action.response && action.response.entities) {
    return _.merge({}, state, action.response.entities)
  }
  
  return state
}

const place = (state = {
  fetching: false,
  id: 1,
  selectedId: 1
}, action) => {
  switch (action.type) {
    case ActionTypes.PLACE.REQUEST:
      return _.merge({}, state, {
        fetching: true
      })
    case ActionTypes.PLACE.SUCCESS:
      return _.merge({}, state, {
        fetching: false,
        id: action.response.result
      })
    case ActionTypes.PLACE.FAILURE:
      return _.merge({}, state, {
        fetching: false
      })
    default:
      return state
  }
}

const rootReducer = combineReducers({
  entities,
  place
})

export default rootReducer
