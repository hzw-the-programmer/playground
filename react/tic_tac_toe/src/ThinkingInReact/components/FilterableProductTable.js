import React, { Component } from 'react'
import SearchBar from './SearchBar'
import ProductTable from './ProductTable'

export default class FilterableProductTable extends Component {
  constructor(props) {
    super(props)
    this.state = {
      filterText: '',
      inStockOnly: false
    }

    this.onFilterTextChange = this.onFilterTextChange.bind(this)
    this.onInStockOnlyChange = this.onInStockOnlyChange.bind(this)
  }

  onFilterTextChange(e) {
    this.setState({
      filterText: e.target.value
    })
  }

  onInStockOnlyChange(e) {
    this.setState({
      inStockOnly: e.target.checked
    })
  }

  render() {
    const { filterText, inStockOnly } = this.state
    return (
      <div>
        <SearchBar filterText={filterText}
                   onFilterTextChange={this.onFilterTextChange}
                   inStockOnly={inStockOnly}
                   onInStockOnlyChange={this.onInStockOnlyChange} />
        <ProductTable filterText={filterText}
                      inStockOnly={inStockOnly}
                      products={this.props.products} />
      </div>
    )
  }
}
