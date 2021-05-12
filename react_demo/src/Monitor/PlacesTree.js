import React, { Component } from 'react'
import { Tree } from 'antd'

const { TreeNode } = Tree

export default class PlacesTree extends Component {
  render() {
    const { places, loadData, onSelect } = this.props
    return (
      <Tree
        loadData={loadData}
        onSelect={onSelect}>
        {this.renderPlaces(places)}
      </Tree>
    )
  }

  renderPlaces = places =>
    places.map(place =>
      <TreeNode key={place.id} title={place.name} dataRef={place}>
        {place.children && this.renderPlaces(place.children)}
      </TreeNode>
    )
}
