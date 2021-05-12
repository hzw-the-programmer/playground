import React from 'react'
import { connect } from 'react-redux'
import { increment, decrement, incrementAsync } from '../../actions/counter'

const counter = ({value, onIncrement, onDecrement, onIncrementAsync}) =>
<div>
    <button onClick={onIncrement}>
        增加
    </button>
    {' '}
    <button onClick={onDecrement}>
        减少
    </button>
    {' '}
    <button onClick={onIncrementAsync}>
        异步增加
    </button>
    <hr />
    <div>
        Clicked: {value} times
    </div>
</div>

const mapStateToProps = state => ({
    value: state.counter.value
})

const mapDispatchToProps = dispatch => ({
    onIncrement: () => dispatch(increment()),
    onDecrement: () => dispatch(decrement()),
    onIncrementAsync: () => dispatch(incrementAsync())
})

export default connect(mapStateToProps, mapDispatchToProps)(counter)
