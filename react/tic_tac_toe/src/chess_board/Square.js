import React from 'react'

const Square = (props) => {
  const { black } = props
  const fill = black ? 'black' : 'white'
  const stroke = black ? 'white' : 'black'
  return (
    <div onClick={props.onClick} style={{
      backgroundColor: fill,
      color: stroke,
      width: '50px',
      height: '50px'
    }}>
      {props.children}
    </div>
  )
}

export default Square
