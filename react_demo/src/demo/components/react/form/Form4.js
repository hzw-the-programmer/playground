import React from 'react'
import { Form, Input } from 'antd'
const FormItem = Form.Item

const LoginForm = Form.create({
  mapPropsToFields(props) {
    console.log('mapPropsToFields', props)
    return {
      username: Form.createFormField({...props.username}),
      password: Form.createFormField({...props.password})
    }
  },
  onFieldsChange(props, changedFields) {
    console.log('onFieldsChange', changedFields)
    props.onChange(changedFields)
  }
})(props => {
  console.log('LoginForm', 'render')
  const { form } = props
  return (
    <Form layout="inline">
      <FormItem label="用户名">
        {form.getFieldDecorator('username', {
          rules: [{required: true, message: '用户名必须填写'}]
        })(<Input />)}
      </FormItem>
      <FormItem label="密码">
        {form.getFieldDecorator('password', {
          rules: [{required: true, message: '密码必须填写'}]
        })(<Input type="password" />)}
      </FormItem>
    </Form>
  )
})

export default class Container extends React.Component {
  state = {
    fields: {
      username: {value: 'hzw'},
      password: {value: 'hzwp'}
    }
  }

  handleChange = changedFields => {
    this.setState(({fields}) => ({
      fields: {
        ...fields,
        ...changedFields
      }
    }))
  }

  render() {
    console.log('Container', 'render')
    const { fields } = this.state
    return (
      <div>
        <LoginForm {...fields} onChange={this.handleChange}/>
        <pre>
          {JSON.stringify(fields, null, 2)}
        </pre>
      </div>
    )
  }
}
