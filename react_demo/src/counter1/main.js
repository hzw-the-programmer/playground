import React from 'react'
import ReactDOM from 'react-dom'
import configureStore from './configureStore'
import { INCREMENT, DECREMENT, INCREMENT_ASYNC } from './actions'
import Counter from './components/Counter'

const store = configureStore()
const action = type => store.dispatch({type})

const render = () => {
  ReactDOM.render(
    <Counter
      value={store.getState()}
      onIncrement={() => action(INCREMENT)}
      onDecrement={() => action(DECREMENT)}
      onIncrementAsync={() => action(INCREMENT_ASYNC)} />,
    document.getElementById('root')
  )
}

render()
store.subscribe(render)
