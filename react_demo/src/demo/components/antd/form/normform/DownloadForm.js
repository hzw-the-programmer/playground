import React from 'react'
import { Button, DatePicker, Form} from 'antd'
const { RangePicker } = DatePicker
const FormItem = Form.Item

class DownloadForm extends React.Component {
  handleSubmit = e => {
    e.preventDefault()

    console.log('before validate')
    
    this.props.form.validateFields((err, values) => {
      console.log('validate')

      if (err) {
        return
      }

      /*
      const a = document.createElement('a')
      a.href = 'http://localhost/download_file.php'
      a.download = 'report.csv'
      a.click()
      */

      fetch('http://localhost/download_file.php')
        .then(response => response.blob())
        .then(blob => {
          const url = window.URL.createObjectURL(blob)
          const a = document.createElement('a')
          a.href = url
          a.download = 'report.csv'
          a.click()
        })
    })

    console.log('after validate')
  }

  render() {
    const { getFieldDecorator } = this.props.form

    return (
      <Form onSubmit={this.handleSubmit} action="http://localhost/download_file.php">
        <FormItem>
          {getFieldDecorator('drange', {
            rules: [{required: true}]
          })(<RangePicker />)}
        </FormItem>
        <FormItem>
          <Button htmlType="submit">
            Submit
          </Button>
        </FormItem>
      </Form>
    )
  }
}

export default Form.create()(DownloadForm)
