import React, { Component } from 'react'
import SearchBar from './SearchBar'
import ProductTable from './ProductTable'

class FilterableProductTable extends Component {
  constructor(props) {
    super(props);

    this.handleFilterTextChange = this.handleFilterTextChange.bind(this);
    this.handleInStockOnlyChange = this.handleInStockOnlyChange.bind(this);

    this.state = {
      filterText: '',
      inStockOnly: false
    };
  }

  handleFilterTextChange(filterText) {
    this.setState({filterText});
  }

  handleInStockOnlyChange(inStockOnly) {
    this.setState({inStockOnly});
  }

  render() {
    return (
      <div>
        <SearchBar onFilterTextChange={this.handleFilterTextChange}
                   onInStockOnlyChange={this.handleInStockOnlyChange}/>
        <ProductTable products={this.props.products}
                      filterText={this.state.filterText}
                      inStockOnly={this.state.inStockOnly} />
      </div>
    );
  }
}

export default FilterableProductTable
