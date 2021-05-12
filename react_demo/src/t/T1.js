import React, { Component } from 'react'
import T2 from './T2'

export default class T1 extends Component {
  constructor(props) {
    super(props)
    this.state = {
      count: 0
    }

    console.log('T1 constructor')
  }

  componentWillMount() {
    console.log('T1 componentWillMount')
  }

  render() {
    console.log('T1 render 1')
    const elem = <T2 count={this.state.count} onClick={this.handleClick} />
    console.log('T1 render 2')
    return elem
  }

  componentDidMount() {
    console.log('T1 componentDidMount')
  }

  componentWillReceiveProps(nextProps) {
    console.log('T1 componentWillReceiveProps', this.props, nextProps)
  }

  shouldComponentUpdate(nextProps, nextState) {
    console.log('T1 shouldComponentUpdate', this.props, nextProps, this.state, nextState)
    return true
  }

  componentWillUpdate(nextProps, nextState) {
    console.log('T1 componentWillUpdate', this.props, nextProps, this.state, nextState)
  }

  componentDidUpdate(preProps, preState) {
    console.log('T1 componentDidUpdate', this.props, preProps, this.state, preState)
  }

  handleClick = e => {
    const count = this.state.count + 1
    this.setState({count})
  }
  
}