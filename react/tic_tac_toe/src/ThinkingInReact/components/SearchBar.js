import React from 'react'

const SearchBar = (props) => (
  <form>
    <input type="text"
           placeholder="Search..."
           value={props.filterText}
           onChange={props.onFilterTextChange} />
    <p>
      <input type="checkbox"
             checked={props.inStockOnly}
             onChange={props.onInStockOnlyChange} />
      {' '}
      Only show products in stock
    </p>
  </form>
)

export default SearchBar
