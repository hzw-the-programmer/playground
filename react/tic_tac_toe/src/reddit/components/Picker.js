import React from 'react'

const Picker = (props) => (
  <span>
    <h1>{props.value}</h1>
    <select value={props.value} onChange={props.onChange}>
      {props.options.map(option => (
        <option value={option} key={option}>
          {option}
        </option>
      ))}
    </select>
  </span>
)

export default Picker
