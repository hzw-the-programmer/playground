import { createStore, applyMiddleware } from 'redux'
import logger from 'redux-logger'
import rootReducer from './reducers'

const configureStore = () =>
  createStore(
    rootReducer,
    applyMiddleware(
      logger
    )
  )

  export default configureStore
