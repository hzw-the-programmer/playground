import _ from 'lodash'

export default function paginate({ types, mapActionToKey }) {
  if (!Array.isArray(types) || types.length !== 3) {
    throw new Error('Expected types to be an array of three elements.')
  }
  if (!types.every(t => typeof t === 'string')) {
    throw new Error('Expected types to be strings.')
  }
  if (typeof mapActionToKey !== 'function') {
    throw new Error('Expected mapActionToKey to be a function.')
  }

  const [ requestType, successType, failureType ] = types

  function updatePagination(state = {
    pageCount: 0
  }, action) {
    switch (action.type) {
      case requestType:
        return _.merge({}, state, {
          isFetching: true
        })
      case successType:
        return _.merge({}, state, {
          isFetching: false,
          ids: _.union(state.ids, action.response.result),
          nextPageUrl: action.response.nextPageUrl,
          pageCount: state.pageCount + 1
        })
      case failureType:
        return _.merge({}, state, {
          isFetching: false
        })
      default:
        return state
    }
  }

  return function updatePaginationByKey(state = {}, action) {
    switch (action.type) {
      case requestType:
      case successType:
      case failureType:
        const key = mapActionToKey(action)
        if (typeof key !== 'string') {
          throw new Error('Expected key to be a string.')
        }
        return _.merge({}, state, {
          [key]: updatePagination(state[key], action)
        })
      default:
        return state
    }
  }
}
