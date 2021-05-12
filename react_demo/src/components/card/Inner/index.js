import React from 'react'
import { Card } from 'antd'

export const Control = () => (
  <Card title="之问 何">
    <p
      style={{
        fontSize: 14,
        fontWeight: 500,
        marginBottom: 16,
        color: 'rgba(0, 0, 0, 0.85)'
      }}
    >
      Software Group
    </p>
    <Card
      title="学习Reactjs"
      extra={<a href="#">更多</a>}
      type="inner"
    >
      认真学习Reactjs。
    </Card>
    <Card
      title="学习Redux"
      extra={<a href="#">更多</a>}
      type="inner"
      style={{marginTop: 16}}
    >
      Redux是非常好的。
    </Card>
  </Card>
)

export const Description = () => (
  <div>
    <p>可以放在普通卡片内部，展示多层级结构的信息。</p>
  </div>
)