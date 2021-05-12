import React from 'react'
import { createForm } from 'rc-form';

import { regionStyle } from './styles'

let uuid = 0

class Form2 extends React.Component {
  submit = e => {
    console.log('submit')

    const { form } = this.props

    console.log(form.getFieldsValue())
  }

  add = e => {
    console.log('add')
    
    uuid++

    const { form } = this.props

    let keys = form.getFieldValue('keys')
    keys = keys.concat(uuid)
    form.setFieldsValue({
      keys
    })
  }

  remove = k => {
    const { form } = this.props

    let keys = form.getFieldValue('keys')
    keys = keys.filter(key => key !== k)
    form.setFieldsValue({
      keys
    })
  }

  render() {
    const { form } = this.props

    form.getFieldProps('keys', {
      initialValue: []
    })

    const keys = form.getFieldValue('keys')
    console.log(keys)

    const inputs = keys.map(k => {
      return (
        <div key={k} style={ regionStyle }>
          <input {...form.getFieldProps(`name${k}`)}/>
          <a onClick={() => this.remove(k)}>Delete</a>
        </div>
      )
    })

    return (
      <div>
        {inputs}
        <div style={ regionStyle }>
          <button onClick={this.submit}>Submit</button>
          <button onClick={this.add}>Add</button>
        </div>
      </div>
    )
  }
}

export default createForm()(Form2)
