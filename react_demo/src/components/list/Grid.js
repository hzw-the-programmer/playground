import React from 'react'
import { List, Card } from 'antd'

const data = [{
  title: '锻炼',
  description: '身体健康'
}, {
  title: '记账',
  description: '财务健康'
}, {
  title: '看书',
  description: '精神健康'
}, {
  title: '建筑',
  description: '精神健康'
}, {
  title: '基金',
  description: '财务健康'
}]

export const Control = () => (
  <List
    grid={{gutter: 16, column: 4}}
    dataSource={data}
    renderItem={item => (
      <List.Item>
        <Card title={item.title}>
          {item.description}
        </Card>
      </List.Item>
    )}
  />
)

export const Description = () => (
  <div>
    <p>可以通过设置 List 的 grid 属性来实现栅格列表，column 可设置期望显示的列数。</p>
  </div>
)