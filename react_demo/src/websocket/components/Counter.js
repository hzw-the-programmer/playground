/**
 * Author: Zhiwen He <18676797056@163.com>
 */
import React from 'react'
import { connect } from 'react-redux'

const Counter = ({value, onIncrement, onDecrement, onIncrementAsync}) =>
  <p>
    Clicked: {value} times
    {' '}
    <button onClick={onIncrement}>
      +
    </button>
    {' '}
    <button onClick={onDecrement}>
      -
    </button>
    {' '}
    <button onClick={onIncrementAsync}>
      Increment Async
    </button>
  </p>

function mapStateToProps(state) {
  return {
    value: state
  }
}

function mapDispatchToProps(dispatch) {
  return {
    onIncrement: () => dispatch({type: 'INCREMENT'}),
    onDecrement: () => dispatch({type: 'DECREMENT'}),
    onIncrementAsync: () => dispatch({type: 'INCREMENT_ASYNC'})
  }
}

export default connect(mapStateToProps, mapDispatchToProps)(Counter)
