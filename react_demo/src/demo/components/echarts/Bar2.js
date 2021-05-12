import React, { Component } from 'react'
import ReactEcharts from 'echarts-for-react'

export default class Bar2 extends Component {
  render() {
    const option = {
      series: [{
        name: '邮件营销',
        type: 'line',
        stack: '总量',
        areaStyle: {normal: {}},
        data: [120, 132, 101, 134, 90, 230, 210]
      }, {
        name: '联盟广告',
        type: 'line',
        stack: '总量',
        areaStyle: {normal: {}},
        data: [220, 182, 191, 234, 290, 330, 310]
      }, {
        name: '视频广告',
        type: 'line',
        stack: '总量',
        areaStyle: {normal: {}},
        data: [150, 232, 201, 154, 190, 330, 410]
      },],
      xAxis: [{
        boundaryGap: false,
        data: ['周一', '周二', '周三', '周四', '周五', '周六', '周日']
      },],
      yAxis: [{}],
      legend: {
        data: ['邮件营销', '联盟广告', '视频广告']
      },
      title: {
        text: '堆叠区域图'
      },
      tooltip: {
        trigger: 'axis'
      },
      toolbox: {
        feature: {
          saveAsImage: {}
        }
      }
    }

    return (
      <ReactEcharts
        option={option}
        style={{width: '80%'}}
      />
    )
  }
}
