import React from 'react'
import { List, Avatar } from 'antd'

const data = [{
  title: '锻炼',
  description: '身体健康'
}, {
  title: '记账',
  description: '财务健康'
}, {
  title: '看书',
  description: '精神健康'
}]

export const Control = () => (
  <List
    dataSource={data}
    renderItem={item => (
      <List.Item>
        <List.Item.Meta
          avatar={<Avatar src="https://zos.alipayobjects.com/rmsportal/ODTLcjxAfvqbxHnVXCYX.png"/>}
          title={item.title}
          description={item.description}
        />
      </List.Item>
    )}
  />
)

export const Description = () => (
  <div>
    <p>基础列表。</p>
  </div>
)