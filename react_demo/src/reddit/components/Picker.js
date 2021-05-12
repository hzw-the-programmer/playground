import React from 'react'

const Picker = ({ value, onChange, options }) =>
  <span>
    <h1>{value}</h1>
    <select value={value} onChange={onChange}>
      {options.map(option =>
        <option key={option} value={option}>
          {option}
        </option>)}
    </select>
  </span>

export default Picker
