import React from 'react'
import { Input, Select } from 'antd'

const { Option } = Select

export default class PriceInput extends React.Component {
  constructor(props) {
    super(props)

    console.log('PriceInput', 'constructor', props)

    const value = props.value || {}
    this.state = {
      number: value.number || 0,
      currency: value.currency || 'rmb'
    }
  }

  triggerChange = value => {
    const { onChange } = this.props
    if (onChange) {
      onChange(Object.assign({}, this.state, value))
    }
  }

  handleNumberChange = e => {
    let number = e.target.value

    console.log('PriceInput', 'handleNumberChange', number)
  
    number = parseInt(number || 0, 10)
    if (isNaN(number)) return

    if ('value' in this.props) {
      this.triggerChange({number})
    } else {
      this.setState({number})
    }
  }

  handleCurrencyChange = currency => {
    console.log('PriceInput', 'handleCurrencyChange', currency)

    if ('value' in this.props) {
      this.triggerChange({currency})
    } else {
      this.setState({currency})
    }
  }

  componentWillReceiveProps(nextProps) {
    console.log('PriceInput', 'componentWillReceiveProps', nextProps)

    if ('value' in nextProps) {
      this.setState(nextProps.value)
    }
  }

  shouldComponentUpdate(nextProps, nextState) {
    console.log('PriceInput', 'shouldComponentUpdate', nextProps, nextState)

    if (nextState.number == this.state.number &&
        nextState.currency == this.state.currency) {
      return false
    }
    return true
  }

  render() {
    console.log('PriceInput', 'render')
    
    return (
      <span>
        <Input
          type="text"
          value={this.state.number}
          onChange={this.handleNumberChange}
          style={{width: '65%', marginRight: '3%'}}
        />
        <Select
          value={this.state.currency}
          onChange={this.handleCurrencyChange}
          style={{width: '32%'}}
        >
          <Option value="rmb">RMB</Option>
          <Option value="dollar">Dollar</Option>
        </Select>
      </span>
    )
  }
}
