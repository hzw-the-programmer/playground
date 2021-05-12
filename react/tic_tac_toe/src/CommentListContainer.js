import React, {Component} from 'react'
import CommentList from './CommentList'
import Promised from './Promised'

class CommentListContainer extends Component {
  render() {
    return (
      <CommentList comments={this.props.comments} />
    );
  }
}

export default Promised('comments', CommentListContainer)
