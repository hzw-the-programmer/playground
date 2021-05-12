import React from 'react'

const Posts = (props) => (
  <ul>
    {props.posts.map((post, i) => (
      <li key={i}>{post.title}</li>
    ))}
  </ul>
)

export default Posts
