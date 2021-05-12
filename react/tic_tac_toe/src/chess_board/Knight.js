import React, { Component } from 'react'
import { DragSource } from 'react-dnd'
import { ItemTypes } from './Constants'

const knightSource = {
  beginDrag() {
    return {}
  }
}

function collect(connect, monitor) {
  return {
    connectDragSource: connect.dragSource(),
    isDragging: monitor.isDragging()
  }
}

class Knight extends Component {
  render() {
    const { connectDragSource, isDragging } = this.props
    return connectDragSource(
      <span style={{
        opacity: isDragging ? .5 : 1,
        fontSize: 25,
        fontWeight: 'bold',
        cursor: 'move'
      }}>
        &#9816;
      </span>
    )
  }
}

export default DragSource(ItemTypes.KNIGHT, knightSource, collect)(Knight)
