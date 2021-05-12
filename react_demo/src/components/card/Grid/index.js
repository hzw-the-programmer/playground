import React from 'react'
import { Card } from 'antd'

const gridStyle = {
  width: '25%',
  textAlign: 'center'
}

export const Control = () => (
  <Card title="何之问">
    <Card.Grid style={gridStyle}>锻炼</Card.Grid>
    <Card.Grid style={gridStyle}>记账</Card.Grid>
    <Card.Grid style={gridStyle}>看书</Card.Grid>
    <Card.Grid style={gridStyle}>软件</Card.Grid>
    <Card.Grid style={gridStyle}>基金</Card.Grid>
    <Card.Grid style={gridStyle}>建筑</Card.Grid>
    <Card.Grid style={gridStyle}>装饰</Card.Grid>
  </Card>
)

export const Description = () => (
  <div>
    <p>一种常见的卡片内容区隔模式。</p>
  </div>
)