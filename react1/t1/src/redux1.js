import React, { Component } from 'react'
import ReactDOM from 'react-dom'
import { createStore } from 'redux'

const count = (state = 0, action) => {
    console.log('reducer count', state, action)
    switch (action.type) {
        case 'INCREMENT':
            return ++state
        default:
            return state
    }
}

const repeat = (state = 'r', action) => {
    console.log('reducer repeat', state, action)
    switch (action.type) {
        case 'REPEAT':
            return state + 'r'
        default:
            return state
    }
}

const reducer = (state = {}, action) => ({
    count: count(state.count, action),
    repeat: repeat(state.repeat, action),
})

console.log('before createStore')
const store = createStore(reducer)
console.log('after createStore')

class Provider extends Component {
    constructor(props) {
        super(props)
        console.log('Provider.constructor')
    }

    render() {
        console.log('Provider.render')
        const { children } = this.props

        return (
            <div>
                {children}
            </div>
        )
    }
}

class Counter extends Component {
    constructor(props) {
        super(props)
        console.log('Counter.constructor')
        const { store } = this.props
        this.state = {count: store.getState().count}
        store.subscribe(this.handleStoreChange.bind(this))
        this.handleIncrement = this.handleIncrement.bind(this)
    }

    handleStoreChange() {
        const { store } = this.props
        if (this.state.count == store.getState().count) {
            return
        }
        this.setState({count: store.getState().count})
    }

    handleIncrement() {
        const { store } = this.props
        store.dispatch({type: 'INCREMENT'})
    }

    render() {
        console.log('Counter.render')
        return (
            <div>
                {this.state.count}<br />
                <button onClick={this.handleIncrement}>Increment</button>
            </div>
        )
    }
}

class Repeater extends Component {
    constructor(props) {
        super(props)
        console.log('Repeater.constructor')
        const { store } = this.props
        this.state = {repeat: store.getState().repeat}
        store.subscribe(this.handleStoreChange.bind(this))
        this.handleRepeat = this.handleRepeat.bind(this)
    }

    handleStoreChange() {
        const { store } = this.props
        if (this.state.repeat == store.getState().repeat) {
            return
        }
        this.setState({repeat: store.getState().repeat})
    }

    handleRepeat() {
        const { store } = this.props
        store.dispatch({type: 'REPEAT'})
    }

    render() {
        console.log('Repeater.render')
        return (
            <div>
                {this.state.repeat}<br />
                <button onClick={this.handleRepeat}>Repeat</button>
            </div>
        )
    }
}

ReactDOM.render((
    <Provider>
        <Counter store={store} />
        <Repeater store={store} />
    </Provider>
), document.getElementById('root'))
