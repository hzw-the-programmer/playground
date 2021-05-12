import React from 'react'
import { Card } from 'antd'
import './index.css'

const BorderLess = () => (
  <section className="code-box">
    <section className="code-box-demo">
      <div style={{background: 'rgb(236, 236, 236)', padding: '30px'}}>
        <Card title="锻炼" bordered={false} style={{ width: 300 }}>
          <p>200个俯卧撑</p>
          <p>200个下蹲</p>
          <p>200个哑铃</p>
        </Card>
      </div>
    </section>
    <section className="code-box-meta">
      <div>
        <p>在灰色背景上使用无边框的卡片。</p>
      </div>
    </section>
  </section>
)

export default BorderLess