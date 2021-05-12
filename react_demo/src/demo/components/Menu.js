import React from 'react'
import { Link } from 'react-router-dom'
import { Menu } from 'antd'

import {menus} from '../menus'

function renderMenu(menus, role, key = 0) {
  return menus.map((menu, index) => {
    const nkey = `${key}-${index}`
    let render = true

    if (menu.role == 'ROLE_ADMIN' && role == 'ROLE_USER') {
      render = false
    } else if (menu.role == 'ROLE_SUPER_ADMIN' && (role == 'ROLE_USER' || role == 'ROLE_ADMIN')) {
      render = false
    }

    return render ? (menu.children ? (
      <Menu.SubMenu key={nkey} title={menu.title}>
        {renderMenu(menu.children, role, nkey)}
      </Menu.SubMenu>
    ) : (
      <Menu.Item key={nkey}>
        <Link to={menu.path}>{menu.title}</Link>
      </Menu.Item>
    )) : null
  })
}

export default ({login}) => {
  return (
    <Menu mode="inline">
      {renderMenu(menus, login.role)}
    </Menu>
  )
}
