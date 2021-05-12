import React, { Component } from 'react'
import { BrowserRouter as Router, Link, Route, Redirect } from 'react-router-dom'

const AuthExample = () => (
  <Router>
    <div>
      <ul>
        <li>
          <Link to="/public">
            Public page
          </Link>
        </li>
        <li>
          <Link to="/protected">
            Protected page
          </Link>
        </li>
      </ul>

      <Route path="/public" component={Public} />
      <ProtectedRoute path="/protected" component={Protected} />
      <Route path="/login" component={Login} />
    </div>
  </Router>
)

const Public = () => <h3>Public</h3>

const Protected = () => <h3>Protected</h3>

class Login extends Component {
  render() {
    console.log(this.props.location)
    return (
      <h1>haha</h1>
    )
  }
}

const fakeAuth = {
  isAuthenticated: false
}

const ProtectedRoute = ({ component: Component, ...rest }) => (
  <Route {...rest} render={props => (
    fakeAuth.isAuthenticated ? (
      <Component />
    ) : (
      <Redirect to={{
        pathname: 'login',
        state: {from: props.location}
      }} />
    )
  )}/>
)

export default AuthExample
