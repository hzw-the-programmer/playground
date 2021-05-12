import React, { Component } from 'react'
import { Tree } from 'antd'

const TreeNode = Tree.TreeNode

class PlacesTree extends Component {
  state = {
    loading: true,
    error: null,
    places: null
  }

  onLoadData = treeNode => {
    return new Promise(resolve => {
      const place = treeNode.props.dataRef
      if (place.children) {
        resolve()
        return
      }

      fetch(`${this.props.url}?pid=${place.id}`)
        .then(response => response.json())
        .then(places => {
          place.children = places
          this.setState((preState, props) => ({
            loading: false, places: preState.places
          }))
          resolve()
        })
        .catch(error => {
          this.setState({loading: false, error})
          resolve()
        });
    })
  }

  componentDidMount() {
    fetch(`${this.props.url}?pid=1`)
      .then(response => response.json())
      .then(places => this.setState({loading: false, places}))
      .catch(error => this.setState({loading: false, error}));
  }

  renderTreeNodes = places => places.map(place => {
    if (place.children) {
      return (
        <TreeNode key={place.id} title={place.name} dataRef={place}>
          {this.renderTreeNodes(place.children)}
        </TreeNode>
      )
    }
    return <TreeNode key={place.id} title={place.name} dataRef={place} />
  })

  render() {
    if (this.state.loading) {
      return <span>Loading...</span>;
    } else if (this.state.error) {
      return <span>{this.state.error.message}</span>;
    } else {
      return (
        <Tree loadData={this.onLoadData}>
          {this.renderTreeNodes(this.state.places)}
        </Tree>
      )
    }
  }
}

export default PlacesTree
