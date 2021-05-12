import React, { Component } from 'react'

export default class T2 extends Component {
  constructor(props) {
    super(props)
    this.state = {
      count: 0
    }
    console.log('T2 constructor')
  }

  componentWillMount() {
    console.log('T2 componentWillMount')
  }

  render() {
    console.log('T2 render')
    return (
      <div>
        <span>{this.props.count}</span>
        <button onClick={this.props.onClick}>INC</button>
        <span>{this.state.count}</span>
        <button onClick={this.handleClick}>Inc</button>
      </div>
    )
  }

  componentDidMount() {
    console.log('T2 componentDidMount')
  }

  // is invoked before a mounted component receives new props. 
  componentWillReceiveProps(nextProps) {
    console.log('T2 componentWillReceiveProps', this.props, nextProps)
  }

  shouldComponentUpdate(nextProps, nextState) {
    console.log('T2 shouldComponentUpdate', this.props, nextProps, this.state, nextState)
    return true
  }

  componentWillUpdate(nextProps, nextState) {
    console.log('T2 componentWillUpdate', this.props, nextProps, this.state, nextState)
  }

  componentDidUpdate(preProps, preState) {
    console.log('T2 componentDidUpdate', this.props, preProps, this.state, preState)
  }

  handleClick = e => {
    const count = this.state.count + 1
    this.setState({count})
  }
}