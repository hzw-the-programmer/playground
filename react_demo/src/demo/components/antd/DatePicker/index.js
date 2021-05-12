import React from 'react'

import { LocaleProvider } from 'antd'
import zh_CN from 'antd/lib/locale-provider/zh_CN'

//import moment from 'moment'
//import 'moment/locale/zh-cn'
//moment.locale('zh-cn')

import { DatePicker } from 'antd'
const { RangePicker } = DatePicker

function onChange(value, dateString) {
  console.log('Selected Time: ', value)
  console.log('Formatted Selected Time: ', dateString)
}

function onOk(value) {
  console.log('onOk: ', value)
}

export default () => (
  <LocaleProvider locale={zh_CN}>
    <div>
      <RangePicker
        showTime={{format: 'HH:mm'}}
        format="YYYY-MM-DD HH:mm"
        onChange={onChange}
        onOk={onOk}
      />
    </div>
  </LocaleProvider>
)
