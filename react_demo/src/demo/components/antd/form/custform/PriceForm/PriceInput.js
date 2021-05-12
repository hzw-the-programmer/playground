import React from 'react'
import { Input, Select } from 'antd'
const Option = Select.Option

export default class PriceInput extends React.Component {
  constructor(props) {
    super(props)

    const value = props.value || {}
    this.state = {
      number: value.number || 0,
      currency: value.currency || 'rmb'
    }
  }

  handleCurrencyChange = currency => {
    console.log('handleCurrencyChange: enter ', this.state.currency)
  
    if (!('value' in this.props)) {
      this.setState({currency})
    }
  
    this.triggerChange({currency})
  
    console.log('handleCurrencyChange: exit ', this.state.currency)
  }

  handleNumberChange = e => {
    console.log('handleNumberChange: enter ', e.target.value)
    console.log('handleNumberChange: enter ', e.target.value || 0)
  
    const number = parseInt(e.target.value || 0, 10)
    if (isNaN(number)) {
      return
    }

    if (!('value' in this.props)) {
      this.setState({number})
    }
    
    this.triggerChange({number})
  }

  triggerChange = changedValue => {
    const onChange = this.props.onChange

    if (onChange) {
      onChange(Object.assign({}, this.state, changedValue))
    }
  }

  componentWillReceiveProps(nextProps) {
    console.log('componentWillReceiveProps', nextProps)
    if ('value' in nextProps) {
      this.setState(nextProps.value)
    }
  }

  shouldComponentUpdate(nextProps, nextState) {
    console.log('shouldComponentUpdate', nextProps, nextState)
    if (this.state.number === nextState.number &&
        this.state.currency === nextState.currency) {
      return false
    }
    return true
  }

  componentWillUpdate(nextProps, nextState) {
    console.log('componentWillUpdate', nextProps, nextState)
  }

  componentDidUpdate(prevProps, prevState) {
    console.log('componentDidUpdate', prevProps, prevState)
  }

  render() {
    console.log('render')
    return (
      <span>
        <Input
          style={{width: '65%', marginRight: '3%'}}
          value={this.state.number}
          onChange={this.handleNumberChange}
        />
        <Select
          style={{width: '32%'}}
          value={this.state.currency}
          onChange={this.handleCurrencyChange}
        >
          <Option value="rmb">RMB</Option>
          <Option value="dollar">Dollar</Option>
        </Select>
      </span>
    )
  }
}
