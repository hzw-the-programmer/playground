import React from 'react'
import ReactDOM from 'react-dom'
import {
  BrowserRouter as Router,
  Link,
  Route
} from 'react-router-dom'

const App = () => (
  <div>
    <ul>
      <li><Link to="/">Home</Link></li>
      <li><Link to="/about">About</Link></li>
      <li><Link to="/topics">Topics</Link></li>
    </ul>
    <Route exact path="/" component={ Home } />
    <Route path="/about" component={ About } />
    <Route path="/topics" component={ Topics } />
  </div>
)

const Home = ({ match, location, history }) => {
  // console.log(match, location, history)
  return (
    <h1>Home</h1>
  )
}

const About = () => (
  <h1>About</h1>
)

const Topics = ({ match }) => (
  <div>
    <ul>
      <li><Link to={`${match.url}/rendering`}>Rendering with React</Link></li>
      <li><Link to={`${match.url}/components`}>Components</Link></li>
      <li><Link to={`${match.url}/prop-v-state`}>Prop v. State</Link></li>
    </ul>

    <Route path={`${match.path}/:topicId`} component={ Topic } />
    <Route exact path={match.path} render={() => <h3>Please select a topic.</h3>} />
  </div>
)

const Topic = ({ match }) => (
  <div>
    <h3>{match.params.topicId}</h3>
  </div>
)

ReactDOM.render(
  <Router>
    <App />
  </Router>,
  document.getElementById('root')
)
