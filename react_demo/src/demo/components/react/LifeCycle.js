import React from 'react'

class CC extends React.Component {
  constructor(props) {
    super(props)
    console.log(props.name, 'constructor')
  }

  componentWillMount() {
    console.log(this.props.name, 'componentWillMount')
  }

  render() {
    console.log(this.props.name, 'render')
    return (
      <div>
        <h1>{this.props.name}</h1>
        <button onClick={this.handleClick}>
          click me
        </button>
      </div>
    )
  }

  componentDidMount() {
    console.log(this.props.name, 'componentDidMount')
  }

  componentWillUnmount() {
    console.log(this.props.name, 'componentWillUnmount')
  }

  componentWillReceiveProps(nextProps) {
    console.log(this.props.name, 'componentWillReceiveProps', nextProps)
  }

  shouldComponentUpdate(nextProps, nextState) {
    console.log(this.props.name, 'shouldComponentUpdate', nextProps, nextState)
    return true
  }

  componentWillUpdate(nextProps, nextState) {
    console.log(this.props.name, 'componentWillUpdate', nextProps, nextState)
  }

  componentDidUpdate(prevProps, prevState, snapshot) {
    console.log(this.props.name, 'componentDidUpdate', prevProps, prevState, snapshot)
  }

  handleClick = e => {
    console.log(this.props.name, 'handleClick')
    this.setState({s1: 1})
  }
}

export default class LifeCycle extends React.Component {
  constructor(props) {
    super(props)
    console.log('constructor')
  }

  componentWillMount() {
    console.log('componentWillMount')
  }

  render() {
    console.log('render')
    const cc2 = <CC name="CC2" />
    return (
      <div>
        <CC name="CC1" />
        {cc2}
        <button onClick={this.handleClick}>click me</button>
      </div>
    )
  }

  componentDidMount() {
    console.log('componentDidMount')
  }

  componentWillUnmount() {
    console.log('componentWillUnmount')
  }

  componentWillReceiveProps(nextProps) {
    console.log('componentWillReceiveProps', nextProps)
  }

  shouldComponentUpdate(nextProps, nextState) {
    console.log('shouldComponentUpdate', nextProps, nextState)
    return true
  }

  componentWillUpdate(nextProps, nextState) {
    console.log('componentWillUpdate', nextProps, nextState)
  }

  componentDidUpdate(prevProps, prevState, snapshot) {
    console.log('componentDidUpdate', prevProps, prevState, snapshot)
  }

  handleClick = e => {
    console.log('handleClick')
    this.setState({s1: 1})
  }
}
