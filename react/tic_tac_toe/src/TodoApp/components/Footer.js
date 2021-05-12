import React from 'react'
import FilterLink from '../containers/FilterLink'

const Footer = () => (
  <p>
    Show:
    {' '}
    <FilterLink visibilityFilter="SHOW_ALL">
      All
    </FilterLink>
    {', '}
    <FilterLink visibilityFilter="SHOW_ACTIVE">
      Active
    </FilterLink>
    {', '}
    <FilterLink visibilityFilter="SHOW_COMPLETED">
      Completed
    </FilterLink>
  </p>
)

export default Footer
