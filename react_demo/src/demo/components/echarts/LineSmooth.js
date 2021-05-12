import React, { Component } from 'react'
import ReactEcharts from 'echarts-for-react'

export default class LineSmooth extends Component {
    render() {
        const option = {
            xAxis: {
                type: 'category',
                data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
            },
            yAxis: {
                type: 'value'
            },
            series: [{
                type: 'line',
                data: [820, 932, 901, 934, 1290, 1330, 1320],
                smooth: true
            }]
        }

        return (
            <ReactEcharts
                option={option}
            />
        )
    }
}
