import React from 'react'
import { Row, Col, Card } from 'antd'
import './index.css'

const Column = () => (
  <section className="code-box">
    <section className="code-box-demo">
      <div style={{background: '#ececec', padding: '30px'}}>
        <Row gutter={16}>
          <Col span={8}>
            <Card title="坚持" bordered={false}>锻炼</Card>
          </Col>
          <Col span={8}>
            <Card title="坚持" bordered={false}>记账</Card>
          </Col>
          <Col span={8}>
            <Card title="坚持" bordered={false}>看书</Card>
          </Col>
        </Row>
      </div>
    </section>
    <section className="code-box-meta">
      <div>
        <p>在系统概览页面常常和栅格进行配合。</p>
      </div>
    </section>
  </section>
)

export default Column