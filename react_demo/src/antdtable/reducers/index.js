import { combineReducers } from "redux";

/**
 * Author: Zhiwen He <18676797056@163.com>
 */
function basicTable(state = [], action) {
  switch (action.type) {
    case 'BASIC_TABLE_FETCHED':
      return [...action.payload]
    default:
      return state
  }
}
export default combineReducers({
  basicTable
})
