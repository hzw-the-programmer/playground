import React, { Component } from 'react'
import ReactEcharts from 'echarts-for-react'

export default class AreaBasic extends Component {
    render() {
        const option = {
            xAxis: {
                type: 'category',
                data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
                boundaryGap: false
            },
            yAxis: {
                type: 'value'
            },
            series: [{
                type: 'line',
                data: [820, 932, 901, 934, 1290, 1330, 1320],
                areaStyle: {}
            }]
        }
        return (
            <ReactEcharts
                option={option}
            />
        )
    }
}
