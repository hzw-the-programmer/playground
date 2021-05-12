import React from 'react'
import { Button, Form } from 'antd';
import PriceInput from './PriceInput'

const FormItem = Form.Item

class Container extends React.Component {
  handleSubmit = e => {
    const { form } = this.props

    form.validateFields((err, values) => {
      console.log(err, values)
    })
  }

  checkPrice = (rule, value, callback) => {
    console.log(rule, value)
    if (value.number > 0) {
      callback()
      return
    }
    callback('Price must greater than zero!')
  }

  render() {
    console.log('Container', 'render')
    const { form } = this.props
    return (
      <div>
        <FormItem label="Price">
          {form.getFieldDecorator('price', {
            initialValue: {number: 0, currency: 'dollar'},
            rules: [{validator: this.checkPrice}]
          })(<PriceInput/>)}
        </FormItem>
        <Button onClick={this.handleSubmit}>Submit</Button>
      </div>
    )
  }
}

export default Form.create({
  onFieldsChange(props, changedField) {
    console.log('onFieldsChange', props, changedField)
  }
})(Container)
