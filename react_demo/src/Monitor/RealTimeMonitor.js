import React, { Component } from 'react'
import { Layout } from 'antd'
import PlacesTree from './PlacesTree'
import PlaceChildren from './PlaceChildren'

const { Sider, Content } = Layout

export default class RealTimeMonitor extends Component {
  constructor(props) {
    super(props)
    this.state = {
      places: [{
        id: 1, name: '根', pid: 0, level: 0
      }],
      place: null
    }
  }

  render() {
    const { places, place } = this.state
    const children = place && place.children ? place.children : []

    return (
      <Layout>
        <Sider
          style={{background: '#fff'}}
        >
          <PlacesTree
            places={places}
            loadData={this.handleLoadData}
            onSelect={this.handleOnSelect}
          />
        </Sider>
        <Content>
          <PlaceChildren places={children} />
        </Content>
      </Layout>
    )
  }

  handleLoadData = (treeNode) => {
    const place = treeNode.props.dataRef
    if (place.children) {
      return Promise.resolve()
    }
    return fetch('http://localhost/api/get_places.php?pid=' + place.id)
      .then(response => response.json())
      .then(places => {
        place.children = places
        this.setState({
          places: [...this.state.places]
        })
      })
  }

  handleOnSelect = (selectedKeys, event) => {
    const place = event.node.props.dataRef
    this.setState({
      place
    })
    if (place.children) {
      return
    }
    fetch('http://localhost/api/get_places.php?pid=' + place.id)
      .then(response => response.json())
      .then(places => {
        place.children = places
        this.setState({
          places: [...this.state.places]
        })
      })
  }
}