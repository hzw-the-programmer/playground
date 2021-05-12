import React from 'react'
import { BrowserRouter as Router, Route, Link } from 'react-router-dom'

const Children = () => (
  <Router>
    <ul>
      <ListItemLink to="/somewhere" />
      <ListItemLink to="/somewhere-else" />
    </ul>
  </Router>
)

const ListItemLink = ({ to, ...rest }) => (
  <Route to={to} children={({ match }) => (
    <li className={ match ? 'active' : '' }>
      <Link to={to} {...rest} />
    </li>
  )} />
)

export default Children
