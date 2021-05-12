import React, { Component } from 'react'
import Board from './Board'
import './App.css'

export default class App extends Component {
  constructor(props) {
    super(props)
    this.state = {
      history: [{
        squares: Array(9).fill(null)
      }],
      move: 0
    }
  }

  jumpTo(move) {
    this.setState({
      move
    })
  }

  handleClick(i) {
    const move = this.state.move
    const history = this.state.history.slice(0, move + 1)
    const squares = history[move].squares.slice()
    const xIsNext = move % 2 === 0

    if (calculateWinner(squares) || squares[i]) {
      return
    }

    squares[i] = xIsNext ? 'X' : 'O'

    history.push({squares})

    this.setState({
      history,
      move: move + 1
    })
  }

  render() {
    const move = this.state.move
    const history = this.state.history
    const squares = history[move].squares
    const xIsNext = move % 2 === 0

    const winner = calculateWinner(squares)
    let status
    if (winner) {
      status = 'Winner: ' + winner
    } else {
      status = 'Next player: ' + (xIsNext ? 'X' : 'O')
    }

    const moves = history.map((item, move) => {
      const desc = move
        ? 'Go to move #' + move
        : 'Go to game start'
      return (
        <li key={move}>
          <button onClick={() => this.jumpTo(move)}>{desc}</button>
        </li>
      )
    })

    return (
      <div className="game">
        <div className="game-board">
          <Board squares={squares}
                 onClick={(i) => this.handleClick(i)}/>
        </div>
        <div className="game-info">
          <div>{status}</div>
          <ol>{moves}</ol>
        </div>
      </div>
    )
  }
}

function calculateWinner(squares) {
  const lines = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6]
  ]
  for (let i =0; i < lines.length; i++) {
    const [a, b, c] = lines[i]
    if (squares[a]
       && squares[a] === squares[b]
       && squares[a] === squares[c]) {
      return squares[a]
    }
  }
  return null
}
