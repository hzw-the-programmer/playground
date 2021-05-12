import React from 'react'

class Counter extends React.Component {
  render() {
    const { value, onIncrement, onDecrement, onIncrementIfOdd, onIncrementAsync } = this.props
    return (
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
        <button onClick={onIncrementIfOdd}>
          Increment If Odd
        </button>
        {' '}
        <button onClick={onIncrementAsync}>
          Increment Async
        </button>
      </p>
    )
  }
}

export default Counter