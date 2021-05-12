import React from 'react'
import { Card } from 'antd'
import './index.css'

const Simple = () => (
  <section className="code-box">
    <section className="code-box-demo">
      <Card style={{ width: 300 }}>
        <p>锻炼</p>
        <p>记账</p>
        <p>看书</p>
      </Card>
    </section>
    <section className="code-box-meta">
      <div>
        <p>只包含内容区域。</p>
      </div>
    </section>
  </section>
)

export default Simple