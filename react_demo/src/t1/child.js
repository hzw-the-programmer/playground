import React from 'react'
import ReactDOM from 'react-dom'

const Parent = ({ children }) => (
  // <h1>Parent</h1>
  <div>
    <h1>Parent</h1>
    {children}
  </div>
)

const Child = () => (
  <h2>Child</h2>
)

const App = () => (
  <Parent>
    <div>
      <Child />
    </div>
  </Parent>
)

ReactDOM.render(
  <App />,
  document.getElementById('root')
)
