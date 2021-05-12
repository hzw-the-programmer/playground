import React from 'react'
import { Switch, Route, Redirect } from 'react-router-dom'

import {menus} from '../menus'
import Index from './Index'

export default class Routes extends React.Component {
  renderRoutes = (menus, role, key = 0) => {
    return menus.map((menu, index) => {
      const nkey = `${key}-${index}`
      let render = true

      if (menu.role == 'ROLE_ADMIN' && role == 'ROLE_USER') {
        render = false
      } else if (menu.role == 'ROLE_SUPER_ADMIN' && (role == 'ROLE_USER' || role == 'ROLE_ADMIN')) {
        render = false
      }

      return render ? (menu.children ? (
        this.renderRoutes(menu.children, role, nkey)
      ) : (
        <Route key={nkey} path={menu.path} component={menu.component} />
      )) : null
    })
  }

  render() {
    return (
      <Switch>
        <Route path="/index" component={Index} />
        {this.renderRoutes(menus, this.props.login.role)}
        <Redirect exact from="/" to="/index" />
        <Redirect to="/404" />
      </Switch>
    )
  }
}
