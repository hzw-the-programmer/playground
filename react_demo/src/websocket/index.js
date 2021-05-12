/**
 * Author: Zhiwen He <18676797056@163.com>
 */
import React from 'react'
import ReactDOM from 'react-dom'
import { createStore, applyMiddleware } from 'redux'
import { Provider } from 'react-redux'
import createSagaMiddleware from 'redux-saga'
import rootReducer from './reducers'
import rootSaga from './sagas'
import Counter from './Components/Counter'
import Chat from './Components/Chat'

const sagaMiddleware = createSagaMiddleware()
const store = createStore(
  rootReducer,
  applyMiddleware(sagaMiddleware)
)
sagaMiddleware.run(rootSaga)
console.log('after run rootSaga')
store.dispatch({type: 'SEND', payload: 'Hello Zhiwen He'})
store.dispatch({type: 'SEND', payload: 'Hello Zhiwen He Again'})

ReactDOM.render(
  <Provider store={store}>
    <div>
      <Counter />
      <Chat />
    </div>
  </Provider>,
  document.getElementById('root')
)
