import React from 'react'
import { Row, Col } from 'antd'
import './BasicGrid.css'

const BasicGrid = () => (
  <div>
    <Row>
      <Col span={12} className="odd">col-12</Col>
      <Col span={12} className="even">col-12</Col>
    </Row>
    <Row>
      <Col span={8} className="odd">col-8</Col>
      <Col span={8} className="even">col-8</Col>
      <Col span={8} className="odd">col-8</Col>
    </Row>
    <Row>
      <Col span={6} className="odd">col-6</Col>
      <Col span={6} className="even">col-6</Col>
      <Col span={6} className="odd">col-6</Col>
      <Col span={6} className="even">col-6</Col>
    </Row>
  </div>
)

export default BasicGrid
