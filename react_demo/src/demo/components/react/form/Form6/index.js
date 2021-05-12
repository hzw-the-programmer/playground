import React from 'react'
import { Form } from 'antd'

class Form6 extends React.Component {
  constructor(props) {
    super(props)
  }

  handleGet = e => {
    const { form } = this.props
    console.log('Form6', 'handleGet', form.getFieldValue('name'))
  }

  componentWillReceiveProps(nextProps) {
    console.log('Form6', 'componentWillReceiveProps', nextProps)
  }

  shouldComponentUpdate(nextProps, nextState) {
    console.log('Form6', 'shouldComponentUpdate', nextProps, nextState)
    return false
  }

  render() {
    console.log('Form6', 'render')

    const { form } = this.props
    return (
      <div>
        <label>
          Name: <input {...form.getFieldProps('name')}/>
        </label>
        <button onClick={this.handleGet}>get</button>
      </div>
    )
  }
}

export default Form.create()(Form6)
