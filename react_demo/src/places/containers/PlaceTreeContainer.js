import React, { Component } from 'react'
import { connect } from 'react-redux'
import * as actions from '../actions'
import PlaceTree from '../components/PlaceTree'

class PlaceTreeContainer extends Component {
  render() {
    const { id, selectedId, places } = this.props
    
    return (
      <PlaceTree
        id={id}
        selectedId={selectedId}
        places={places} />
    )
  }
}

const mapStateToProps = state => ({
  id: state.place.id,
  selectedId: state.place.selectedId,
  fetching: state.place.fetching,
  places: state.entities.places
})

const mapDispatchToProps = dispatch => ({
  loadPlace: id => dispatch(actions.loadPlace(id))
})

export default connect(mapStateToProps, mapDispatchToProps)(PlaceTreeContainer)
