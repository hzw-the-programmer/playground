import React from 'react'
import ReactDOM from 'react-dom'
import { Provider } from 'react-redux'
import { HashRouter as Router, Switch, Route } from 'react-router-dom'

import configureStore from './store'
import Login from './components/Login'
import NotFound from './components/NotFound'
import Root from './components/Root'

const store = configureStore()

ReactDOM.render(
  <Provider store={store}>
    <Router>
      <Switch>
        <Route path="/login" component={Login}/>
        <Route path="/404" component={NotFound}/>
        <Route component={Root} />
      </Switch>
    </Router>
  </Provider>,
  document.getElementById('root')
)
