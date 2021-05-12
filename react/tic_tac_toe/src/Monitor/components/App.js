import React, { Component } from 'react'
import PlacesTree from './PlacesTree'

export default class App extends Component {
  constructor(props) {
    super(props)
    this.state = {
      places: [
        {id: 1, name: '根', pid: 0, level: 0},
      ]
    }
  }

  render() {
    const { places } = this.state
    return (
      <PlacesTree places={places}
                  onLoadData={this.handlePlacesTreeLoadData}
                  onSelect={this.handlePlacesTreeSelect} />
    )
  }

  handlePlacesTreeLoadData = treeNode => {
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

  handlePlacesTreeSelect = (selectedKeys, e) => {
    console.log(selectedKeys)
  }
}