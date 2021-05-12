import React, {Component} from 'react'
import {Tree} from 'antd'

const TreeNode = Tree.TreeNode;

const treeData = [{
  title: '0-0',
  key: '0-0',
  children: [{
    title: '0-0-0',
    key: '0-0-0',
    children: [{
      title: '0-0-0-0',
      key: '0-0-0-0'
    }, {
      title: '0-0-0-1',
      key: '0-0-0-1'
    }, {
      title: '0-0-0-2',
      key: '0-0-0-2'
    }]
  }, {
    title: '0-0-1',
    key: '0-0-1',
    children: [{
      title: '0-0-1-0',
      key: '0-0-1-0'
    }, {
      title: '0-0-1-1',
      key: '0-0-1-1'
    }, {
      title: '0-0-1-2',
      key: '0-0-1-2'
    }]
  }, {
    title: '0-0-2',
    key: '0-0-2'
  }]
}, {
  title: '0-1',
  key: '0-1',
  children: [{
    title: '0-1-0',
    key: '0-1-0'
  }, {
    title: '0-1-1',
    key: '0-1-1'
  }]
}, {
  title: '0-2',
  key: '0-2'
}, {
  title: '0-3',
  key: '0-3'
}];

class TreeControlled extends Component {
  state = {
    selectedKeys: [],
    checkedKeys: [],
    expandedKeys: [],
    autoExpandParent: true
  }

  onSelect = (selectedKeys, info) => {
    console.log('onSelect', selectedKeys, info);
    this.setState({selectedKeys});
  }

  onCheck = (checkedKeys, info) => {
    console.log('onCheck', checkedKeys, info);
    this.setState({checkedKeys});
  }

  onExpand = expandedKeys => {
    console.log('onExpand', expandedKeys);
    this.setState({expandedKeys, autoExpandParent: false});
  }

  renderTreeNodes(treeData) {
    return treeData.map(node => {
      if (node.children) {
        return (
          <TreeNode title={node.title} key={node.key}>
            {this.renderTreeNodes(node.children)}
          </TreeNode>
        );
      }
      return (
        <TreeNode {...node} />
      );
    });
  }

  render() {
    return (
      <Tree
        selectedKeys={this.state.selectedKeys}
        onSelect={this.onSelect}
        checkable
        onCheck={this.onCheck}
        checkedKeys={this.state.checkedKeys}
        onExpand={this.onExpand}
        expandedKeys={this.state.expandedKeys}
        autoExpandParent={this.state.autoExpandParent}>
        {this.renderTreeNodes(treeData)}
      </Tree>
    );
  }
}

export default TreeControlled
