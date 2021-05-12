import React from 'react'
import ReactDOM from 'react-dom'

import { createStore, applyMiddleware } from 'redux'
import createSagaMiddleware from 'redux-saga'

import rootReducer from './reducers'
import rootSaga from './sagas'

import Counter from './components/Counter'

const action = type => store.dispatch({type})

const sagaMiddleware = createSagaMiddleware()
const store = createStore(rootReducer, applyMiddleware(sagaMiddleware))
sagaMiddleware.run(rootSaga)

const render = () => ReactDOM.render(
  <Counter
    value={store.getState()}
    onIncrement={() => action('INCREMENT')}
    onDecrement={() => action('DECREMENT')}
    onIncrementIfOdd={() => action('INCREMENT_IF_ODD')}
    onIncrementAsync={() => action('INCREMENT_ASYNC')}
  />,
  document.getElementById('root')
)

store.subscribe(() => {
  console.log('render')
  render()
})

render()