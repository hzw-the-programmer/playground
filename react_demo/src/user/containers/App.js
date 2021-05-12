import React from 'react'
import { connect } from 'react-redux'
import { LOAD_USER } from '../actions'
import User from '../components/User'

const App = ({ loading, user, error, loadUser }) => {
  let content
  if (loading) {
    content = <h1>Loading</h1>
  } else if (error) {
    content = <h1>{error}</h1>
  } else if (!user) {
    content = <h1>No data</h1>
  } else {
    content = <User {...user} />
  }

  return (
    <div>
      <button onClick={() => loadUser(1)}>
        Load User
      </button>
      <hr />
      {content}
    </div>
  )
}

const mapStateToProps = state => ({
  ...state
})

const mapDispatchToProps = dispatch => ({
  loadUser: id => dispatch({type: LOAD_USER, id})
})

export default connect(mapStateToProps, mapDispatchToProps)(App)
