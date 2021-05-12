import React, { Component } from 'react'
import { List, Card, Avatar } from 'antd'

export default class PlaceChildren extends Component {
  render() {
    const { places } = this.props
    
    return (
      <List
        grid={{gutter: 16, column: 4}}
        dataSource={places}
        renderItem={place => (
          <List.Item>
            <Card
              style={{ width: 300 }}
            >
              <Card.Meta
                title={place.name}
                avatar={<Avatar style={{backgroundColor: 'red'}} />}
              />
            </Card>
          </List.Item>
        )}
      />
    )
  }
}