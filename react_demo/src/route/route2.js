import React from 'react'
import ReactDOM from 'react-dom'
import { BrowserRouter, Link, Route } from 'react-router-dom'

const App = () => (
  <BrowserRouter>
    <div>
      <nav>
        <Link to="/tacos">Tacos</Link>
      </nav>
      <div>
        <Route path="/tacos" component={ Tacos } />
      </div>
    </div>
  </BrowserRouter>
)

const Tacos = ({ match, location, history }) => {
  console.log(match, location, history)
  return (
    <div>
      <Link to={ match.url + '/carnitas' }>Carnitas</Link>
      <Route path={ match.url + '/carnitas' } component={ Carnitas } />
    </div>
  )
}

const Carnitas = () => (
  <h2>Carnitas</h2>
)

ReactDOM.render(
  <App />,
  document.getElementById('root')
)
