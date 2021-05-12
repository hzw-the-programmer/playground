import React from 'react'
import { connect } from 'react-redux'
import { Form, Button } from 'antd'
const FormItem = Form.Item

import { submitPriceForm } from '../../../../../actions'

import PriceInput from './PriceInput'

class PriceForm extends React.Component {
  handleSubmit = e => {
    e.preventDefault()

    this.props.form.validateFields((err, values) => {
      if (!err) {
        console.log('Received values of form: ', values)
        this.props.submit(values)
      }
    })
  }

  render() {
    const { getFieldDecorator } = this.props.form

    return (
      <Form layout="inline" onSubmit={this.handleSubmit}>
        <FormItem label="Price">
          {getFieldDecorator('price', {
            initialValue: this.props.price
          })(<PriceInput />)}
        </FormItem>
        <FormItem>
          <Button type="primary" htmlType="submit">Submit</Button>
        </FormItem>
      </Form>
    )
  }
}

const mapStateToProps = state => ({
  price: {number: 321, currency: 'rmb'}
})

const mapDispatchToProps = dispatch => ({
  submit: values => dispatch(submitPriceForm({values}))
})

export default connect(mapStateToProps, mapDispatchToProps)(Form.create()(PriceForm))
