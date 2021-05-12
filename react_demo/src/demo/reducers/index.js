import {combineReducers} from 'redux'

import userMgr from './userMgr'
import counter from './counter'

export default combineReducers({
  userMgr,
  counter,
})
