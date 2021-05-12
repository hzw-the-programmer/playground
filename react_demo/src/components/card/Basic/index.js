import React from 'react'
import { Card } from 'antd'
import './index.css'

const Basic = () => (
  <section className="code-box">
    <section className="code-box-demo">
      <Card
        title="标题"
        extra={<a href="#">更多</a>}
        style={{width: 300}}
      >
        <p>学习Reactjs</p>
        <p>学习Redux</p>
        <p>学习antd</p>
      </Card>
    </section>
    <section className="code-box-meta">
      <div class="code-box-title">
        <a href="#">典型卡片</a>
      </div>
      <div>
        <p>包含标题、内容、操作区域。</p>
      </div>
    </section>
  </section>
)

export default Basic