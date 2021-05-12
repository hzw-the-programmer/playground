import React from 'react'
import ReactDOM from 'react-dom'
import { BrowserRouter as Router,
         Route,
         Link } from 'react-router-dom'
import Reddit from './reddit/containers/Reddit'
import TodoApp from './TodoApp/index'
import TicTacToe from './TicTacToe/index'
import ThinkingInReact from './ThinkingInReact/index'
import Monitor from './Monitor/index'

ReactDOM.render(
  <Router>
    <div>
      <Link to="/reddit">reddit</Link>
      {' '}
      <Link to="/todoapp">TodoApp</Link>
      {' '}
      <Link to="/tictactoe">TicTacToe</Link>
      {' '}
      <Link to="/thinkinginreact">Thinking In React</Link>
      {' '}
      <Link to="/monitor">Monitor</Link>

      <Route path="/reddit" component={Reddit} />
      <Route path="/todoapp" component={TodoApp} />
      <Route path="/tictactoe" component={TicTacToe} />
      <Route path="/thinkinginreact" component={ThinkingInReact} />
      <Route path="/monitor" component={Monitor} />
    </div>
  </Router>,
  document.getElementById('root')
)

// import main from './chess_board/index'
// import main from './react_route/index'
// import main from './reddit/index'

// main()
// import React from 'react'
// import ReactDOM from 'react-dom'
// import Station from './Station'
//
// const channels = [{
//   id: 0,
//   type: 0,
//   color: 'red',
//   row: 0,
//   column: 0
// }, {
//   id: 1,
//   type: 1,
//   color: 'red',
//   row: 1,
//   column: 1
// }, {
//   id: 2,
//   type: 1,
//   color: 'green',
//   row: 2,
//   column: 2
// }, {
//   id: 3,
//   type: 2,
//   color: 'red',
//   row: 3,
//   column: 3
// }, {
//   id: 4,
//   type: 3,
//   color: 'purple',
//   row: 5,
//   column: 3
// }]
//
// ReactDOM.render(
//   <Station channels={channels} padding={4}
//            onChannelClick={(channel) => console.log('onChannelClick', channel)}
//            onChannelMouseEnter={(channel) => console.log('onChannelMouseEnter', channel)}
//            onChannelMouseLeave={(channel) => console.log('onChannelMouseLeave', channel)} />,
//   document.getElementById('root')
// );
