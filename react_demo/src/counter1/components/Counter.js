import React from 'react'

const Counter = ({value, onIncrement, onDecrement, onIncrementAsync }) =>
  <div>
    <button onClick={onIncrement}>
      Increment
    </button>
    {' '}
    <button onClick={onDecrement}>
      Decrement
    </button>
    {' '}
    <button onClick={onIncrementAsync}>
      Increment Async
    </button>
    <hr />
    <div>
      Clicked: {value} times
    </div>
  </div>

  export default Counter