import React, { Component } from 'react'
import { Tree } from 'antd'
import 'antd/lib/tree/style/css'

const TreeNode = Tree.TreeNode

export default class PlacesTree extends Component {
  render() {
    const { places, onLoadData, onSelect } = this.props

    return (
      <Tree loadData={onLoadData}
            onSelect={onSelect}>
        {this.renderTreeNodes(places)}
      </Tree>
    )
  }

  renderTreeNodes(places) {
    return places.map(place => {
      if (place.children) {
        return (
          <TreeNode key={place.id} title={place.name} dataRef={place}>
            {this.renderTreeNodes(place.children)}
          </TreeNode>
        )
      }
      return <TreeNode key={place.id} title={place.name} dataRef={place} />
    })
  }
}