import React, { Component } from 'react'
import { Tree } from 'antd'

const TreeNode = Tree.TreeNode

class TreeAsync extends Component {
  state = {
    treeData: [
      {title: 'Plant 1', key: 0},
      {title: 'Plant 2', key: 1},
      {title: 'Leaf', key: 2, isLeaf: true}
    ]
  }

  onLoadData = treeNode => {
    return new Promise(resolve => {
      if (treeNode.props.children) {
        resolve()
        return
      }

      setTimeout(() => {
        treeNode.props.dataRef.children = [
          {title: 'workshop 1', key: `${treeNode.props.eventKey}-0`},
          {title: 'workshop 2', key: `${treeNode.props.eventKey}-1`},
        ]
        this.setState({
          treeData: [...this.state.treeData]
        })
        resolve()
      }, 1000)
    });
  }

  renderTreeNodes = treeData => {
    return treeData.map(item => {
      if (item.children) {
        return (
          <TreeNode title={item.title} key={item.key} dataRef={item}>
            {this.renderTreeNodes(item.children)}
          </TreeNode>
        )
      }
      return <TreeNode {...item} dataRef={item} />
    });
  }

  render = () => (
    <Tree loadData={this.onLoadData}>
      {this.renderTreeNodes(this.state.treeData)}
    </Tree>
  )
}

export default TreeAsync
