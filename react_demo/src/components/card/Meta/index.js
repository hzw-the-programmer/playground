import React from 'react'
import { Card, Icon, Avatar } from 'antd'
import './index.css'

const Meta = () => (
  <section className="code-box">
    <section className="code-box-demo">
      <Card
        cover={<img alt="bear" src="https://gw.alipayobjects.com/zos/rmsportal/JiqGstEfoWAOHiTxclqi.png" />}
        actions={[<Icon type="setting" />, <Icon type="edit" />, <Icon type="ellipsis" />]}
        style={{ width: 300 }}
      >
        <Card.Meta
          avatar={<Avatar src="https://zos.alipayobjects.com/rmsportal/ODTLcjxAfvqbxHnVXCYX.png" />}
          title="何之问"
          description="注定成功"
        />
      </Card>
    </section>
    <section className="code-box-meta">
      <div>
        <p>一种支持封面、头像、标题和描述信息的卡片。</p>
      </div>
    </section>
  </section>
)

export default Meta