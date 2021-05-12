import React from 'react'
import { TimePicker } from 'antd'
import moment from 'moment'
import { Form, Button } from 'antd'

/*export default */class WorkTime extends React.Component {
    constructor(props) {
        super(props)
        console.log('WorkTime', 'constructor', props)

        const periods = props.value || []
        this.state = {periods}
    }

    handleAdd = () => {
        console.log('WorkTime', 'handleAdd')
        const periods = [...this.state.periods]
        const now = moment().unix()
        periods.push([now, now])
        
        if ('onChange' in this.props) {
            this.props.onChange(periods)
        } else {
            this.setState({periods})
        }
    }

    handleDelete(i) {
        console.log('WorkTime', 'handleDelete')
        const periods = [...this.state.periods]
        periods.splice(i, 1)

        if ('onChange' in this.props) {
            this.props.onChange(periods)
        } else {
            this.setState({periods})
        }
    }

    handleChange(i, j, time, timeString) {
        console.log('WorkTime', 'handleChange')
        const periods = [...this.state.periods]
        periods[i] = [...periods[i]]
        periods[i][j] = time.unix()

        if ('onChange' in this.props) {
            this.props.onChange(periods)
        } else {
            this.setState({periods})
        }
    }

    componentWillReceiveProps(nextProps) {
        console.log('WorkTime', 'componentWillReceiveProps', nextProps)
        if ('value' in this.props) {
            this.setState({periods: nextProps.value})
        }
    }

    render() {
        console.log('WorkTime', 'render')
        const periods = this.state.periods

        return (
            <div>
                <Button onClick={this.handleAdd}>Add</Button>
                {periods.map((period, i) => {
                    return (
                        <div key={i}>
                            {period.map((time, j) => (
                                <TimePicker
                                  key={j}
                                  value={moment.unix(time)}
                                  onChange={this.handleChange.bind(this, i, j)} />
                            ))}
                            <Button onClick={this.handleDelete.bind(this, i)}>Delete</Button>
                        </div>
                    )
                })}
            </div>
        )
    }
}

/*export default */class Container extends React.Component {
    constructor(props) {
        super(props)
        console.log('Container', 'constructor', props)
        const now = moment().unix()
        this.state = {periods: []}
    }

    handleChange = periods => {
        console.log('Container', 'handleChange')
        this.setState({periods})
    }

    render() {
        console.log('Container', 'render')
        const periods = this.state.periods

        return (
            <WorkTime value={periods} onChange={this.handleChange} />
        )
    }
}

class FormContainer extends React.Component {
    constructor(props) {
        super(props)
        console.log('FormContainer', 'constructor', props)
    }

    handleSubmit = e => {
        e.preventDefault()
        this.props.form.validateFields((err, values) => {
            console.log('FormContainer', 'handleSubmit', err, values)
            if (!err) {
                console.log('submit')
            }
        })
    }

    checkPeriods = (rule, value, callback) => {
        console.log('checkPeriods', rule, value, callback)
        for (let i = 0; i < value.length; i++) {
            if (value[i][0] >= value[i][1]) {
                callback(`start time is greater than end time in period ${i}`)
                return
            }
        }
        callback()
    }

    render() {
        console.log('FormContainer', 'render')
        const { getFieldDecorator } = this.props.form

        return (
            <Form onSubmit={this.handleSubmit}>
                <Form.Item label="Periods">
                    {getFieldDecorator('periods', {
                        rules: [{validator: this.checkPeriods}]
                    })(<WorkTime />)}
                </Form.Item>
                <Form.Item>
                    <Button type="primary" htmlType="submit">Submit</Button>
                </Form.Item>
            </Form>
        )
    }
}

export default Form.create()(FormContainer)
