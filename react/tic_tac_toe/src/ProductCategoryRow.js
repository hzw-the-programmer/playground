import React, { Component } from 'react'

class ProductCategoryRow extends Component {
  render() {
    const category = this.props.category;
    return (
      <tr>
        <td colSpan="2">{category}</td>
      </tr>
    );
  }
}

export default ProductCategoryRow
