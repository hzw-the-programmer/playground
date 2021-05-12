import React, { Component } from 'react'
import { Layout } from 'antd'
import PlaceTreeContainer from './PlaceTreeContainer'
import PlaceChildren from '../components/PlaceChildren'

const { Sider, Content } = Layout

export default class App extends Component {
  render() {
    return (
      <Layout>
        <Sider style={{backgroundColor: 'white'}}>
          <PlaceTreeContainer />
        </Sider>
        <Content>
          {/* <PlaceChildren /> */}
        </Content>
      </Layout>
    )
  }
}
