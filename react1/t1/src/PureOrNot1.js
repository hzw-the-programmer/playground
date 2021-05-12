import React, { Component, PureComponent } from 'react'
import ReactDOM from 'react-dom'

import {createStore, applyMiddleware} from 'redux'
import createSagaMiddleware from 'redux-saga'
import { delay, takeLatest, cancelled, put } from 'redux-saga/effects'

import { isEqual } from 'lodash'

let i = 0
const people = (state = [], action) => {
    switch (action.type) {
        case 'ADD_PEOPLE':
            return [...state, { id: ++i, name: action.payload}]
        case 'savePeople':
            return action.payload
        default:
            return state
    }
}
const count = (state = 0, action) => {
    switch (action.type) {
        case 'INCREMENT':
            return state + 1
        default:
            return state
    }
}

const reducers = (state = {}, action) => ({
    people: people(state.people, action),
    count: count(state.count, action),
})

function* load(action) {
    try {
        yield delay(2000)
        yield put({type: 'savePeople', payload: [
            {id: 1, name: 'hzw'}
        ]})
        console.log('after put')
    } finally {
        if (yield cancelled()) {
            console.log('cancelled')
        }
    }
}

const sagas = function*() {
    yield takeLatest('LOAD', load)
}

const sagaMiddleware = createSagaMiddleware()
const store = createStore(reducers, applyMiddleware(sagaMiddleware))
sagaMiddleware.run(sagas)

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

// class Counter extends Component {
class Counter extends PureComponent {
    constructor(props) {
        super(props)
        console.log('Counter.constructor')
        this.handleIncrement = this.handleIncrement.bind(this)
        this.handleStoreChange = this.handleStoreChange.bind(this)
        const { store } = this.props
        const { count } = store.getState()
        this.state = {
            count
        }
        store.subscribe(this.handleStoreChange)
    }

    handleStoreChange() {
        const { store } = this.props
        const { count } = store.getState()
        this.setState({
            count,
        })
    }

    handleIncrement() {
        const { store: { dispatch } } = this.props
        dispatch({type: 'INCREMENT'})
    }

    render() {
        console.log('Counter.render')
        const { store } = this.props
        const { count } = store.getState()
        return (
            <div>
                {count}<button onClick={this.handleIncrement}>Increment</button>
            </div>
        )
    }
}

class AddPeople extends Component {
    constructor(props) {
        super(props)
        console.log('AddPeople.constructor')
        this.state = {
            name: '',
        }
        this.onChange = this.onChange.bind(this)
        this.handleAdd = this.handleAdd.bind(this)
    }

    onChange(event) {
        const { target: { value }} = event
        this.setState({
            name: value
        })
    }

    handleAdd() {
        const { name } = this.state
        const { store: { dispatch } } = this.props
        dispatch({ type: 'ADD_PEOPLE', payload: name})
    }

    render() {
        console.log('AddPeople.render')
        const { name } = this.state
        return (
            <div>
                <input value={name} onChange={this.onChange} />
                <button onClick={this.handleAdd}>Add</button>
            </div>
        )
    }
}

class Table extends Component {
// class Table extends PureComponent {
    constructor(props) {
        super(props)
        console.log('Table.constructor')
        const { store } = this.props
        const { people } = store.getState()
        this.state = {
            people,
        }
        this.handleStoreChange = this.handleStoreChange.bind(this)
        store.subscribe(this.handleStoreChange)
    }

    handleStoreChange() {
        const { store } = this.props
        const { people } = store.getState()
        this.setState({
            people,
        })
    }

    shouldComponentUpdate(nextProps, nextState) {
        return !isEqual(this.state, nextState)
    }

    render() {
        console.log('Table.render')
        const { people } = this.state
        return (
            <table>
                <tbody>
                    {people.map(person => (
                        <tr key={person.id}>
                            <td>{person.id}</td>
                            <td>{person.name}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
        )
    }
}

const handleLoad = () => {
    store.dispatch({type: 'LOAD'})
}

ReactDOM.render((
    <Provider>
        <Counter store={store} />
        <AddPeople store={store} />
        <Table store={store} />
        <button onClick={handleLoad}>Load</button>
    </Provider>
), document.getElementById('root'));
