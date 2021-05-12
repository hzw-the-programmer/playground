import React, {Component} from 'react'
import {Tree} from 'antd'

const TreeNode = Tree.TreeNode;
const gData = [];

const x = 3;
const y = 2;
const z = 1;

const generateData = (_level, _preKey, _tns) => {
  const preKey = _preKey || '0';
  const tns = _tns || gData;

  let children = [];
  for (let i = 0; i < x; i++) {
    const key = `${preKey}-${i}`;
    tns.push({key, title: key});
    if (i < y) {
      children.push(key);
    }
  }

  if (_level < 0) {
    return;
  }
  const level = _level - 1;
  children.forEach((key, i) => {
    tns[i].children = [];
    generateData(level, key, tns[i].children);
  });
};

generateData(z);

class TreeDraggable extends Component {
  state = {gData};

  onDragEnter = info => {
    console.log('onDragEnter', info);
  }

  onDrop = info => {
    console.log('onDrop', info);

    const loop = (data, key, callback) => {
      data.forEach((item, index, arr) => {
        if (item.key === key) {
          return callback(item, index, arr);
        }
        if (item.children) {
          loop(item.children, key, callback);
        }
      });
    };

    const data = [...this.state.gData];

    const dragKey = info.dragNode.props.eventKey;
    let dragObj;
    loop(data, dragKey, (item, index, arr) => {
      arr.splice(index, 1);
      dragObj = item;
    });

    const dropKey = info.node.props.eventKey;

    if (info.dropToGap) {
      let ar;
      let i;
      loop(data, dropKey, (item, index, arr) => {
        ar = arr;
        i = index;
      });

      const dropPos = info.node.props.pos.split('-');
      const dropPosition = info.dropPosition - Number(dropPos[dropPos.length - 1]);
      if (dropPosition === -1) {
        ar.splice(i, 0, dragObj);
      } else {
        ar.splice(i + 1, 0, dragObj);
      }
    } else {
      loop(data, dropKey, item => {
        item.children = item.children || [];
        item.children.push(dragObj);
      });
    }

    this.setState({gData: data});
  }

  render() {
    const loop = data => data.map(node => {
      if (node.children && node.children.length) {
        return (
          <TreeNode key={node.key} title={node.title}>
            {loop(node.children)}
          </TreeNode>
        );
      }
      return (
        <TreeNode key={node.key} title={node.title} />
      );
    });

    return (
      <Tree
        draggable
        onDragEnter={this.onDragEnter}
        onDrop={this.onDrop}>
        {loop(this.state.gData)}
      </Tree>
    );
  }
}

export default TreeDraggable
