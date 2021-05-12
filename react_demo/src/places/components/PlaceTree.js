import React, { Component } from 'react'
import { Tree } from 'antd'

const { TreeNode } = Tree

export default class PlaceTree extends Component {
  renderTreeNodes(ids) {
    const { places } = this.props

    return ids.map(id => {
      const place = places[id]
      if (!place) return

      return (
        <TreeNode key={place.id} title={place.name} id={place.id}>
          {place.children && this.renderTreeNodes(place.children)}
        </TreeNode>
      )
    }).filter(node => node)
  }

  render = () => {
    const { id, selectedId } = this.props

    return (
      <Tree selectedKeys={[`${selectedId}`]}>
        {this.renderTreeNodes([id])}
      </Tree>
    )
  }
}
