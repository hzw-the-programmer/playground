import React, { Component } from 'react'
import { List, Card, Avatar } from 'antd'
import { connect } from 'react-redux'

class PlaceChildren extends Component {
  render() {
    const { loading = false, place } = this.props
    const children = place && place.children ? place.children : []

    return (
      <div style={{margin: '1rem', border: '.2rem solid #f7f7f9', padding: '1rem'}}>
        <List
          grid={{gutter: 16, column: 4}}
          loading={loading}
          dataSource={children}
          renderItem={place => (
            <List.Item>
              <Card>
                <Card.Meta
                  title={place.name}
                  avatar={<Avatar style={{backgroundColor: place.color}} />}
                />
              </Card>
            </List.Item>
          )}
        />
      </div>
    )
  }
}

const mapStateToProps = state => ({
  place: state.placeTree.place
})

export default connect(mapStateToProps)(PlaceChildren)
