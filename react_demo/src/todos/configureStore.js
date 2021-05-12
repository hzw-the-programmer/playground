import { createStore, applyMiddleware } from 'redux'
import logger from 'redux-logger'
import reducer from './reducers'

const configureStore = () => {
  const store = createStore(
    reducer,
    applyMiddleware(logger)
  )
  return store
}

export default configureStore