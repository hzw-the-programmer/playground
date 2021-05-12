import React from 'react'

function CommentList({comments}) {
  return (
    <ul className="comment-box">
      {comments.map((comment, i) => (
        <li key={`reponse-${i}`} className="comment-item">
          <p className="comment-item-author">{comment.author}</p>
          <p className="comment-item-content">{comment.content}</p>
        </li>
      ))}
    </ul>
  );
}

export default CommentList
