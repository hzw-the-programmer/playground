import React, { Component } from 'react'
import ReactEcharts from 'echarts-for-react'

export default class Bar extends Component {
  render() {
    const option = {
      title: {
        text: 'echarts入门'
      },
      legend: {
        data: ['销量', '销量2']
      },
      grid: {},
      xAxis: {
        data: ['衬衫', '羊毛衫', '雪纺衫', '裤子', '高跟鞋', '袜子']
      },
      yAxis: {},
      series: [{
        name: '销量',
        type: 'bar',
        data: [5, 20, 36, 10, 10, 20]
      }, {
        name: '销量2',
        type: 'bar',
        data: [5, 20, 36, 10, 10, 20]
      }, {
        type: 'line',
        data: [5, 20, 36, 10, 10, 20]
      },]
    }

    return (
      <ReactEcharts
        option={option}
        style={{width: '80%'}} />
    )
  }
}