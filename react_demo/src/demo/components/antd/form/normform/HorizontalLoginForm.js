import React from 'react'

import { Form, Icon, Input, Button } from 'antd'
const FormItem = Form.Item

const locale = {
  username: '用户名',
  password: '密码',
  login: '登录'
}

function hasErrors(fieldsError) {
  return Object.keys(fieldsError).some(field => fieldsError[field])
}

class HorizontalLoginForm extends React.Component {
  constructor(props) {
    super(props)
    console.log('constructor')
  }

  componentWillMount() {
    console.log('componentWillMount')
  }

  componentDidMount() {
    this.props.form.validateFields()
    console.log('componentDidMount')
  }

  componentWillUnmount() {
    console.log('componentWillUnmount')
  }

  componentWillReceiveProps(nextProps) {
    console.log('componentWillReceiveProps', nextProps)
  }

  shouldComponentUpdate(nextProps, nextState) {
    console.log('componentShouldUpdate', nextProps, nextState)
    return true
  }

  componentWillUpdate(nextProps, nextState) {
    console.log('componentWillUpdate', nextProps, nextState)
  }

  componentDidUpdate(prevProps, prevState, snapshot) {
    console.log('componentDidUpdate', prevProps, prevState, snapshot)
  }

  handleSubmit = e => {
    e.preventDefault()
    this.props.form.validateFields((err, values) => {
      if (!err) {
        console.log('Received values of form: ', values)
      }
    })
  }

  render() {
    console.log('render')

    const {
      isFieldTouched,
      getFieldError,
      getFieldDecorator,
      getFieldsError,
      getFieldInstance,
      getFieldValue,
    } = this.props.form

    console.log(getFieldsError())

    console.log('username instance', getFieldInstance('username'))
    console.log('username value', getFieldValue('username'))
    console.log('username error', getFieldError('username'))
    const username = getFieldDecorator('username', {
      rules: [{required: true, message: 'Please input your username!'}],
      initialValue: 'zhiwenhe'
    })(
      <Input prefix={<Icon type="user" style={{color: 'rgba(0, 0, 0, .25)'}} />} placeholder={locale.username} />
    )
    console.log('username instance', getFieldInstance('username'))
    console.log('username value', getFieldValue('username'))
    console.log('username error', getFieldError('username'))

    console.log('password', getFieldInstance('password'))
    const password = getFieldDecorator('password', {
      rules: [{required: true, message: 'Please input your Password!'}]
    })(
      <Input prefix={<Icon type="lock" style={{color: 'rgba(0, 0, 0, .25)'}} />} type="password" placeholder={locale.password} />
    )
    console.log('password', getFieldInstance('password'))

    console.log(getFieldsError())

    const usernameError = isFieldTouched('username') && getFieldError('username')
    const passwordError = isFieldTouched('password') && getFieldError('password')

    return (
      <Form layout="inline" onSubmit={this.handleSubmit}>
        <FormItem
          validateStatus={usernameError ? 'error' : ''}
          help={usernameError || ''}
        >
          {username}
        </FormItem>

        <FormItem
          validateStatus={passwordError ? 'error' : ''}
          help={passwordError || ''}
        >
          {password}
        </FormItem>

        <FormItem>
          <Button
            type="primary"
            htmlType="submit"
            disabled={hasErrors(getFieldsError())}
          >
            {locale.login}
          </Button>
        </FormItem>
      </Form>
    )
  }
}

export default Form.create()(HorizontalLoginForm)
