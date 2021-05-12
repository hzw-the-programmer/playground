import React from 'react'
import { Cascader } from 'antd'

export default class ChannelPicker extends React.Component {
  loadData = selectedOptions => {
    console.log(selectedOptions)
  }

  render() {
    return (
      <Cascader
        loadData={this.loadData}
      />
    )
  }
}
