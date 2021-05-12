import { connect } from 'react-redux'
import { setVisibilityFilter } from '../actions'
import Link from '../components/Link'

const mapStateToProps = (state, ownProps) => ({
  active: state.visibilityFilter === ownProps.visibilityFilter
})

const mapDispatchToProps = (dispatch, ownProps) => ({
  onClick: () => {
    dispatch(setVisibilityFilter(ownProps.visibilityFilter))
  }
})

const FilterLink = connect(mapStateToProps, mapDispatchToProps)(Link)

export default FilterLink
