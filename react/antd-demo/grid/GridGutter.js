import React from 'react'
import { Row, Col } from 'antd'
import './GridGutter.css'

const GridGutter = () => (
  <div className="gutter-example">
    <Row gutter={16}>
      <Col span={6} className="gutter-row">
        <div className="gutter-box">
          col-6
        </div>
      </Col>
      <Col span={6} className="gutter-row">
        <div className="gutter-box">
          col-6
        </div>
      </Col>
      <Col span={6} className="gutter-row">
        <div className="gutter-box">
          col-6
        </div>
      </Col>
      <Col span={6} className="gutter-row">
        <div className="gutter-box">
          col-6
        </div>
      </Col>
    </Row>
  </div>
)

export default GridGutter
