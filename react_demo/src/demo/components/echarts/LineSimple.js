import React, { Component } from 'react'
import ReactEcharts from 'echarts-for-react'

export default class LineSimple extends Component {
    render() {
        const option = {
            xAxis: {
                type: 'category',
                data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
                //boundaryGap: false
            },
            yAxis: {
                type: 'value'
            },
            series: [{
                type: 'line',
                data: [820, 932, 901, 934, 1290, 1330, 1320],
                //label: {normal: {show: true}},
                //areaStyle: {normal: {}}
            }],
            //tooltip: {
            //    axisPointer: {
            //        type: 'cross'
            //    }
            //}
        }

        return (
            <ReactEcharts
                option={option}
            />
        )
    }
}
