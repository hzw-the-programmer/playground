import React, {Component} from 'react';
import {Tree} from 'antd';

const TreeNode = Tree.TreeNode;

class TreeBasic extends Component {
  onSelect = (selectedKeys, info) => {
    console.log('selected', selectedKeys, info);
  }

  onCheck = (checkedKeys, info) => {
    console.log('checked', checkedKeys, info);
  }

  render() {
    return (
      <Tree
        onSelect={this.onSelect}
        defaultExpandedKeys={['0-0-0', '0-0-1']}
        defaultSelectedKeys={['0-0-0']}
        defaultCheckedKeys={['0-0-1']}
        checkable
        onCheck={this.onCheck}>
        <TreeNode title="parent 1" key="0-0">
          <TreeNode title="parent 1-0" key="0-0-0">
            <TreeNode title="leaf 1-0-0" key="0-0-0-0" />
            <TreeNode title="leaf 1-0-1" key="0-0-0-1" />
          </TreeNode>
          <TreeNode title="parent 1-1" key="0-0-1">
            <TreeNode title="leaf 1-1-0" key="0-0-1-0" />
            <TreeNode title="leaf 1-1-1" key="0-0-1-1" />
          </TreeNode>
        </TreeNode>
        <TreeNode title="parent 2" key="0-1">
          <TreeNode title="parent 2-0" key="0-1-0" />
          <TreeNode title="parent 2-1" key="0-1-1" />
        </TreeNode>
      </Tree>
    );
  }
}

export default TreeBasic
