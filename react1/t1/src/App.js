import React, {Component, PureComponent, useState} from 'react';
import logo from './logo.svg';
import './App.css';

// function App() {
//   return (
//     <div className="App">
//       <header className="App-header">
//         <img src={logo} className="App-logo" alt="logo" />
//         <p>
//           Edit <code>src/App.js</code> and save to reload.
//         </p>
//         <a
//           className="App-link"
//           href="https://reactjs.org"
//           target="_blank"
//           rel="noopener noreferrer"
//         >
//           Learn React
//         </a>
//       </header>
//     </div>
//   );
// }


function Count() {
  const [count, setCount] = useState(0)
  console.log(`Count: ${count}`)
  return (
    <di>
      <p>you clicked {count} times</p>
      <button onClick={() => setCount(count + 1)}>click</button>
    </di>
  )
}

class Com extends Component {
  render() {
    console.log('Com.render')
    return <h1>com</h1>
  }
}

class Com1 extends Component {
  render() {
    console.log('Com1.render')
    return <h1>com1</h1>
  }

  shouldComponentUpdate(nextProps, nextState) {
    return this.props != nextProps
  }
}

class Pure extends PureComponent {
  render() {
    console.log('Pure.render')
    return <h1>pure</h1>
  }
}

class Child extends Component {
  constructor(props) {
    super(props)
    var {id} = props
    console.log(`Child${id}.constructor`)
    this.add = this.add.bind(this)
    this.state = {count: 0}
  }

  add() {
    this.setState({count: 0})
  }

  render() {
    var {id} = this.props
    console.log(`Child${id}.render`)
    return <button onClick={this.add}>{id}</button>
  }

  shouldComponentUpdate(nextProps, nextState) {
    var {id} = this.props
    console.log(`Child${id}.shouldComponentUpdate`, nextProps, nextState, this.props, this.state, this.props == nextProps, this.state == nextState)
    if (id == 1) return false
    return true
  }

  componentDidMount() {
    var {id} = this.props
    console.log(`Child${id}.componentDidMount`)
  }

  componentWillUnmount() {
    var {id} = this.props
    console.log(`Child${id}.componentWillUnmount`)
  }

  componentDidUpdate(prevProps, prevState, snapshot) {
    var {id} = this.props
    console.log(`Child${id}.componentDidUpdate`, prevProps, prevState, this.props, this.state)
  }
}

class App extends Component {
  constructor(props) {
    super(props)
    console.log('App.constructor')
    this.add = this.add.bind(this)
    this.state = {count: 0}
  }

  add() {
    // this.setState({count: ++this.state.count})
    this.setState({count: 0})
  }

  render() {
    console.log('App.render')
    var child = <div><Child id="1"/></div>
    var pure = <Pure id="1"/>
    if (this.state.count % 3 != 2) {
      child = <div><Child id="1"/><Child id="2"/></div>
      pure = <Pure id ="2"/>
    }
    return (
      <div>
        {child}
        <button onClick={this.add}>Add</button>
        <Count/>
        {pure}
        <Com id="1"/>
        <Com1 id="1"/>
      </div>
    )
  }

  shouldComponentUpdate(nextProps, nextState) {
    console.log('App.shouldComponentUpdate', nextProps, nextState, this.props, this.state, this.props == nextProps, this.state == nextState)
    return true
    // return false
  }

  componentDidMount() {
    console.log('App.componentDidMount')
  }

  componentWillUnmount() {
    console.log('App.componentDidMount')
  }

  componentDidUpdate(prevProps, prevState, snapshot) {
    var {id} = this.props
    console.log(`App.componentDidUpdate`, prevProps, prevState, this.props, this.state)
  }
}

export default App;
