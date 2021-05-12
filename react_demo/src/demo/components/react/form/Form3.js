import React from 'react'
import { createForm } from "rc-form";
import { Modal } from 'antd'

import { regionStyle, errorStyle } from './styles'

class Form3 extends React.Component {
  state = {
    visible: false
  }

  open = e => {
    this.setState({visible: true})
  }

  onCancel = e => {
    this.setState({visible: false})
  }

  onSubmit = e => {
    e.preventDefault()
    const { form } = this.props

    //form.validateFieldsAndScroll((err, values) => {
    form.validateFields((err, values) => {
      if (err) return
      this.form.submit()
    })
  }

  render() {
    const { form } = this.props
    
    const usernameProps = form.getFieldProps('username', {
      initialValue: '',
      rules: [{required: true}]
    })
    const usernameErrors = form.getFieldError('username')

    const passwordProps = form.getFieldProps('password', {
      initialValue: '',
      rules: [{required: true}]
    })
    const passwordErrors = form.getFieldError('password')

    return (
      <div style={{margin: 20}}>
        <h2>modal</h2>
        <button onClick={this.open} style={regionStyle}>open</button>
        <Modal
          visible={this.state.visible}
          onCancel={this.onCancel}
          title="Hansome HZW"
          bodyStyle={{
            height: 200,
            overflow: 'auto'
          }}
        >
          <form onSubmit={this.onSubmit} ref={form => this.form = form} action="http://localhost/download_file.php">
            <div>
              <input {...usernameProps}/>
              {usernameErrors ? <span style={errorStyle}>{usernameErrors.join(',')}</span> : null}
            </div>
            <div>
              <input {...passwordProps} type="password"/>
              {passwordErrors ? <span style={errorStyle}>{passwordErrors.join(',')}</span> : null}
            </div>
            <div style={{marginTop: 300}}>
              <button>Submit</button>
            </div>
          </form>
        </Modal>
      </div>
    )
  }
}

export default createForm()(Form3)
