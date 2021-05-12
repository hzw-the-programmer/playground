import React from 'react'
import { createForm } from 'rc-form'

class Form1 extends React.Component {
  handleClick = e => {
    const { form } = this.props
    const { validateFields } = form

    validateFields((err, values) => {
      console.log(err, values)
    })
  }
  render() {
    const { form } = this.props

    const usernameProps = form.getFieldProps('username', {
      initialValue: 'hzw'
    });
    const passwordProps = form.getFieldProps('password', {
      rules: [{required: true}]
    });

    console.log(usernameProps)
    console.log(passwordProps)

    const usernameErr = form.getFieldError('username')
    const passwordErr = form.getFieldError('password')

    console.log(usernameErr)
    console.log(passwordErr)

    const username = form.getFieldValue('username')
    const password = form.getFieldValue('password')

    console.log(username)
    console.log(password)

    console.log(form.getFieldsValue())

    return (
      <div>
        <input {...usernameProps}/>
        <input type="password" {...passwordProps}/>
        <button onClick={this.handleClick}>Submit</button>
      </div>
    )
  }
}

export default createForm()(Form1)
