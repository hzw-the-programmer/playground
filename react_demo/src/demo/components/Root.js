import React from 'react'
import {connect} from 'react-redux'
import {Redirect} from 'react-router-dom'

import {Layout} from 'antd'
const {Sider, Header, Content, Footer} = Layout

import Menu from './Menu'
import Routes from './Routes'

class Root extends React.Component {
  render() {
    const login = this.props.login
    if (!login) return <Redirect to="/login" />

    return (
      <Layout>
        <Sider style={{background: '#fff'}}>
          <Menu login={login} />
        </Sider>
        <Layout>
          <Header style={{background: '#fff'}}>Header</Header>
          <Content>
            <Routes login={login} />
          </Content>
          <Footer>Footer</Footer>
        </Layout>
      </Layout>
    )
  }
}

const mapStateToProps = state => {
  return {
    login: state.userMgr.login
  }
}

export default connect(mapStateToProps)(Root)
