import React from 'react'
import ProductCategoryRow from './ProductCategoryRow'
import ProductRow from './ProductRow'

const ProductTable = ({ filterText, inStockOnly, products }) => {
  const rows = []
  let currentCategory = null
  products.forEach(product => {
    if (product.name.indexOf(filterText) === -1) {
      return
    }

    if (inStockOnly && !product.stocked) {
      return
    }

    if (currentCategory !== product.category) {
      rows.push(
        <ProductCategoryRow
          category={product.category}
          key={product.category} />
      )
    }

    rows.push(
      <ProductRow
        product={product}
        key={product.name} />
    )
    currentCategory = product.category
  })

  return (
    <table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Price</th>
        </tr>
      </thead>
      <tbody>
        {rows}
      </tbody>
    </table>
  )
}

export default ProductTable
