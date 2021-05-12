import React, { Component } from 'react'
import { connect } from 'react-redux'
import Picker from '../components/Picker'
import Posts from '../components/Posts'
import {
  fetchPostsIfNeeded,
  selectSubreddit,
  invalidateSubreddit
} from '../actions'

class App extends Component {
  render() {
    const { selectedSubreddit, isFetching, posts, lastUpdated } = this.props

    return (
      <div>
        <Picker
          value={selectedSubreddit}
          onChange={this.handleChange}
          options={['reactjs', 'frontend']}
        />
        <p>
          {lastUpdated &&
            <span>
              Last updated at {new Date(lastUpdated).toLocaleTimeString()}
              {' '}
            </span>}
          {!isFetching &&
            <a href="#" onClick={this.handleRefreshClick}>
              Refresh
            </a>}
        </p>
        {posts.length === 0 && isFetching && <h2>Loading...</h2>}
        {posts.length === 0 && !isFetching && <h2>Empty.</h2>}
        {posts.length > 0 &&
          <div style={{opacity: isFetching ? 0.5 : 1}}>
            <Posts posts={posts} />
          </div>}
      </div>
    )
  }

  componentDidMount() {
    const { dispatch, selectedSubreddit } = this.props
    dispatch(fetchPostsIfNeeded(selectedSubreddit))
  }

  handleChange = e => {
    const { dispatch } = this.props
    dispatch(selectSubreddit(e.target.value))
    dispatch(fetchPostsIfNeeded(e.target.value))
  }

  handleRefreshClick = e => {
    e.preventDefault()
    const { selectedSubreddit, dispatch } = this.props
    dispatch(invalidateSubreddit(selectedSubreddit))
    dispatch(fetchPostsIfNeeded(selectedSubreddit))
  }
}

const mapStateToProps = state => {
  const { selectedSubreddit, postsBySubreddit } = state

  const {
    isFetching,
    items: posts,
    lastUpdated
  } = postsBySubreddit[selectedSubreddit] || {
    isFetching: false,
    items: []
  }

  return {
    selectedSubreddit,
    isFetching,
    posts,
    lastUpdated
  }
}

export default connect(mapStateToProps)(App)
