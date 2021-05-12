import { createStore, applyMiddleware } from 'redux'
import thunkMiddleware from 'redux-thunk'
import logger from 'redux-logger'
import rootReducer from './reducers'

const configureStore = () => createStore(
  rootReducer,
  applyMiddleware(
    thunkMiddleware,
    logger
  )
)

export default configureStore
