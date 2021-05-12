import React from 'react'
import { Card } from 'antd'
import './index.css'

const { Meta } = Card

const Flexible = () => (
  <section className="code-box">
    <section className="code-box-demo">
      <Card
        hoverable
        cover={<img alt="beauty" src="https://os.alipayobjects.com/rmsportal/QBnOOoLaAfKPirc.png" />}
        style={{ width: 240 }}
      >
        <Meta
          title="Europe Street beat"
          description="www.instagram.com"
        />
      </Card>
    </section>
    <section className="code-box-meta">
      <div>
        <p>可以利用 Card.Meta 支持更灵活的内容</p>
      </div>
    </section>
  </section>
)

export default Flexible