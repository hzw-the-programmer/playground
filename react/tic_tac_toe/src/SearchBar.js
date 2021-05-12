import React, { Component } from 'react'

class SearchBar extends Component {
  constructor(props) {
    super(props);
    this.handleFilterTextChange = this.handleFilterTextChange.bind(this);
    this.handleInStockOnlyChange= this.handleInStockOnlyChange.bind(this);
  }

  handleFilterTextChange(e) {
    this.props.onFilterTextChange(e.target.value);
  }

  handleInStockOnlyChange(e) {
    this.props.onInStockOnlyChange(e.target.checked);
  }

  render() {
    return (
      <form>
        <input type="text"
               placeholder="Search..."
               onChange={this.handleFilterTextChange}/>
        <p>
          <input type="checkbox"
                 onChange={this.handleInStockOnlyChange} />
          {' '}
          Only show products in stock
        </p>
      </form>
    );
  }
}

export default SearchBar
