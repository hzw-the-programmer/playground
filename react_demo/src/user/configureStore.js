import { createStore, applyMiddleware } from 'redux'
import createSagaMiddleware from 'redux-saga'
import logger from 'redux-logger'
import reducer from './reducers'
import saga from './sagas'

const configureStore = () => {
  const sagaMiddleware = createSagaMiddleware()
  const store = createStore(
    reducer,
    applyMiddleware(
      sagaMiddleware,
      logger
    )
  )
  sagaMiddleware.run(saga)
  return store
}

export default configureStore
