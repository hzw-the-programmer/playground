import React from 'react'

const Counter = ({ value, onIncrement, onDecrement, onIncrementIfOdd, onIncrementAsync }) =>
  <p>
    Clicked: {value} times.
    {' '}
    <button onClick={onIncrement}>
      +
    </button>
    {' '}
    <button onClick={onDecrement}>
      -
    </button>
    {' '}
    <button onClick={onIncrementIfOdd}>
      Increment if odd
    </button>
    {' '}
    <button onClick={onIncrementAsync}>
      Increment async
    </button>
  </p>

  export default Counter