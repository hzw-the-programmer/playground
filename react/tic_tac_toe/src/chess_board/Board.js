import React, { Component } from 'react'
import { DragDropContext } from 'react-dnd'
import HTML5Backend from 'react-dnd-html5-backend'
import Square from './Square'
import Knight from './Knight'
import { moveKnight } from './Game'

class Board extends Component {
  renderSquare = i => {
    const row = Math.floor(i / 8)
    const column = i % 8
    const black = (row + column) % 2 === 1
    const [knightRow, knightColumn] = this.props.knightPosition
    const piece = row === knightRow && column === knightColumn ? <Knight /> : null

    return (
      <Square key={i} black={black} onClick={() => moveKnight(row, column)}>
        {piece}
      </Square>
    )
  }

  render() {
    const squares = []
    for (let i = 0; i < 64; i++) {
      squares.push(this.renderSquare(i))
    }

    return (
      <div style={{
        width: '400px',
        display: 'flex',
        flexWrap: 'wrap'
      }}>
        {squares}
      </div>
    )
  }
}

export default DragDropContext(HTML5Backend)(Board)
