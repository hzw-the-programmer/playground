import React from 'react'
import { Row, Col }  from 'antd'
import './index.css'

export const Control = () => (
  <div>
    <Row>
      <Col span={12}>学习Reactjs</Col>
      <Col span={12}>学习Redux</Col>
    </Row>
    <Row>
      <Col span={8}>锻炼</Col>
      <Col span={8}>记账</Col>
      <Col span={8}>看书</Col>
    </Row>
    <Row>
      <Col span={6}>基金</Col>
      <Col span={6}>建筑</Col>
      <Col span={6}>翻译</Col>
      <Col span={6}>游泳</Col>
    </Row>
  </div>
)

export const Description = () => (
  <div>
    <p>使用单一的一组 `Row` 和 `Col` 栅格组件，就可以创建一个基本的栅格系统，所有列（Col）必须放在 `Row` 内。</p>
  </div>
)