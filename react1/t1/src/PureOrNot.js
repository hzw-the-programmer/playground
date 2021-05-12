import React, {Component, PureComponent} from "react"

class Pure1 extends PureComponent {
    constructor(props) {
        super(props)
        console.log(`Pure1.constructor`)
        this.state = {s: 1}
    }

    render() {
        console.log(`Pure1.render`)
        return <button onClick={() => this.setState({s: 1})}>Pure1 click</button>
    }
}

class Pure2 extends PureComponent {
    constructor(props) {
        super(props)
        console.log(`Pure2.constructor`)
        this.state = {s: {}}
    }

    render() {
        console.log(`Pure2.render`)
        return <button onClick={() => this.setState({s: {}})}>Pure2 click</button>
    }
}

class NotPure1 extends Component {
    constructor(props) {
        super(props)
        console.log(`NotPure1.constructor`)
        this.state = {s: 1}
    }

    render() {
        console.log(`NotPure1.render`)
        return <button onClick={() => this.setState({s: 1})}>NotPure1 click</button>
    }

    shouldComponentUpdate(nextProps, nextState) {
        console.log('NotPure1.shouldComponentUpdate', nextProps, nextState, this.props, this.state, this.props == nextProps, this.state == nextState)
        return true
    }
}

class NotPure2 extends Component {
    constructor(props) {
        super(props)
        console.log(`NotPure2.constructor`)
        this.state = {s: {}}
    }

    render() {
        console.log(`NotPure2.render`)
        return <button onClick={() => this.setState({s: {}})}>NotPure2 click</button>
    }

    shouldComponentUpdate(nextProps, nextState) {
        console.log('NotPure2.shouldComponentUpdate', nextProps, nextState, this.props, this.state, this.props == nextProps, this.state == nextState)
        if (this.props != nextProps && Object.keys(this.props.p).length == 0 && Object.keys(nextProps.p).length == 0) return false
        if (this.state != nextState && Object.keys(this.state.s).length == 0 && Object.keys(nextState.s).length == 0) return false
    }
}

class PureOrNot extends Component {
    constructor(props) {
        super(props)
        console.log(`PureOrNot.constructor`)
        this.state = {s: 1}
    }

    render() {
        console.log(`PureOrNot.render`)
        const p = {}
        return (
            <div>
                <Pure1 p="1"/>
                <NotPure1 p="1"/>
                <Pure2 p={p}/>
                <NotPure2 p={p}/>
                <button onClick={() => this.setState({s: 1})}>PureOrNot click</button>
            </div>
        )
    }

    shouldComponentUpdate(nextProps, nextState) {
        console.log('PureOrNot.shouldComponentUpdate', nextProps, nextState, this.props, this.state, this.props == nextProps, this.state == nextState)
        return true
    }
}

export default PureOrNot
