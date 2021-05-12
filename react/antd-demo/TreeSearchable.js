import React, { Component } from 'react'
import { Tree, Input }  from 'antd'

const TreeNode = Tree.TreeNode
const Search = Input.Search

const x = 3
const y = 2
const z = 2
const gData = []

const generateData = (_level, _preKey, _tns) => {
  const preKey = _preKey || '0'
  const tns = _tns || gData

  const children = []
  for (let i = 0; i < x; i++) {
    const key = `${preKey}-${i}`
    const child = {key, title: key}
    tns.push(child)
    if (i < y) {
      children.push(child)
    }
  }

  if (_level < 0) {
    return
  }

  const level = _level - 1
  children.forEach(child => {
    child.children = []
    generateData(level, child.key, child.children)
  })
}
generateData(z)

class TreeSearchable extends React.Component {

  state = {
    searchValue: '',
    expandedKeys: [],
    autoExpandParent: true
  }

  onExpand = expandedKeys => {
    // console.log(expandedKeys)
    this.setState({
      expandedKeys,
      autoExpandParent: false
    })
  }

  onChange = e => {
    const searchValue = e.target.value

    const expandedKeys = []
    const findExpandedKeys = (tree, parent) => {
      tree.forEach(item => {
        if (item.children) {
          findExpandedKeys(item.children, item)
        } else if (item.title.indexOf(searchValue) > -1) {
          if (parent && expandedKeys.indexOf(parent.key) === -1) {
            expandedKeys.push(parent.key)
          }
        }
      })
    }
    if (searchValue !== '') {
      findExpandedKeys(gData)
    }
    // console.log(expandedKeys)

    this.setState({
      searchValue,
      expandedKeys,
      autoExpandParent: true
    })
  }

  render() {
    const { searchValue, expandedKeys, autoExpandParent } = this.state

    const loop = data => data.map(item => {
      const index = item.title.indexOf(searchValue)
      const beforeStr = item.title.substr(0, index)
      const afterStr = item.title.substr(index + searchValue.length)
      const title = index > -1 ?
        <span>
          {beforeStr}
          <span style={{color: 'red'}}>{searchValue}</span>
          {afterStr}
        </span> :
        <span>{item.title}</span>

      if (item.children) {
        return (
          <TreeNode key={item.key} title={title}>
            {loop(item.children)}
          </TreeNode>
        )
      }

      return <TreeNode key={item.key} title={title} />
    })

    return (
      <div>
        <Search style={{marginBottom: 8}} placeholder="Search" onChange={this.onChange} />
        <Tree expandedKeys={expandedKeys}
              autoExpandParent={autoExpandParent}
              onExpand={this.onExpand}>
          {loop(gData)}
        </Tree>
      </div>
    )
  }
}

export default TreeSearchable
