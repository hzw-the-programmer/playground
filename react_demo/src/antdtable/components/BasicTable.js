import React from 'react'
import { connect } from 'react-redux'
import { Table, Icon, Divider } from 'antd'

const columns = [{
  title: '姓名',
  dataIndex: 'name',
  key: 'name',
  render: text => <a href="javascript:;">{text}</a>
}, {
  title: '年龄',
  dataIndex: 'age',
  key: 'age'
}, {
  title: '地址',
  dataIndex: 'address',
  key: 'address'
}, {
  title: '操作',
  key: 'action',
  render: (text, record) =>
    <span>
      <a href="javascript:;">Action - {record.name}</a>
      <Divider type="vertical" />
      <a href="javascript:;">Delete</a>
      <Divider type="vertical" />
      <a href="javascript:;" className="ant-dropdown-link">
        More actions <Icon type="down" />
      </a>
    </span>
}]

const BasicTable = ({dataSource}) =>
  <Table columns={columns} rowKey='id' dataSource={dataSource} />

function mapStateToProps(state) {
  return {
    dataSource: state.basicTable
  }
}

export default connect(mapStateToProps)(BasicTable)
