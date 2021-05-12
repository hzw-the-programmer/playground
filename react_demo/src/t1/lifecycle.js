import React, { Component } from 'react'
import ReactDOM from 'react-dom'

class Clock extends Component {
  constructor(props) {
    super(props)
    console.log('    Clock constructor')
    this.date = new Date()
    this.state = {
      date: this.date
    }
    // this.state = {
    //   date: new Date()
    // }
  }

  componentWillMount() {
    console.log('    Clock componentWillMount')
  }

  componentDidMount() {
    console.log('    Clock componentDidMount')
    this.timerId = setInterval(this.tick, 1000)
  }

  componentWillUnmount() {
    console.log('    Clock componentWillUnmount')
    clearInterval(this.timerId)
  }

  componentWillReceiveProps(nextProps) {
    console.log('    Clock componentWillReceiveProps', this.props, nextProps)
  }

  shouldComponentUpdate(nextProps, nextState) {
    console.log('    Clock shouldComponentUpdate', this.props, nextProps, this.state, nextState)
    return this.state.date.getTime() !== nextState.date.getTime()
    // return true
  }

  componentWillUpdate(nextProps, nextState) {
    console.log('    Clock componentWillUpdate', this.props, nextProps, this.state, nextState)
  }

  componentDidUpdate(preProps, preState) {
    console.log('    Clock componentDidUpdate', this.props, preProps, this.state, preState)
  }

  render() {
    console.log('    Clock render')
    return (
      <div>
        <h1>Hello, Zhiwen He</h1>
        <h2>
          It is {this.state.date.toLocaleTimeString()}.
        </h2>
      </div>
    )
  }

  tick = () => this.setState({date: this.date})
  // tick = () => this.setState({date: new Date()})
}

class App extends Component {
  constructor(props) {
    super(props)
    console.log('App constructor')
    this.state = {
      displayClock: true
    }
  }

  componentWillMount() {
    console.log('App componentWillMount')
  }

  componentDidMount() {
    console.log('App componentDidMount')
  }

  componentWillReceiveProps(nextProps) {
    console.log('App componentWillReceiveProps')
  }

  shouldComponentUpdate(nextProps, nextState) {
    console.log('App shouldComponentUpdate')
    return true
  }

  componentWillUpdate(nextProps, nextState) {
    console.log('App componentWillUpdate')
  }

  componentDidUpdate(preProps, preState) {
    console.log('App componentDidUpdate')
  }

  render() {
    const { displayClock } = this.state
    console.log('App render')

    return (
      <div>
        {/* {displayClock && <Clock />} */}
        <Clock />
        <button onClick={this.handleClick}>{displayClock ? '不显示' : '显示'}</button>
      </div>
    )
  }

  handleClick = e => this.setState({displayClock: !this.state.displayClock})
}

ReactDOM.render(
  <App />,
  document.getElementById('root')
)
