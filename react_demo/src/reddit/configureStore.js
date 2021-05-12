import { createStore, applyMiddleware } from 'redux'
import thunk from 'redux-thunk'
import logger from 'redux-logger'
import reducer from './reducers'

const configureStore = () => {
  const store = createStore(
    reducer,
    applyMiddleware(
      thunk,
      logger
    )
  )
  return store
}

export default configureStore
