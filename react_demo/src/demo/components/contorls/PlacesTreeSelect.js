import React, { Component } from 'react'
import { TreeSelect } from 'antd'

import { places } from './places'

class PlacesTreeSelect extends Component {
    renderPlaces(places, maxLevel, selectableLevel) {
        if (places && places[0].level <= maxLevel) {
            const selectable = places[0].level === selectableLevel
            return places.map((place) => {
                return (
                    <TreeSelect.TreeNode
                      key={place.id}
                      title={place.name}
                      value={place.id + ''}
                      selectable={selectable}
                    >
                        {this.renderPlaces(place.children, maxLevel, selectableLevel)}
                    </TreeSelect.TreeNode>
                )
            })
        }
    }

    render() {
        const {places, maxLevel, selectableLevel, ...props} = this.props
        return (
            <TreeSelect {...props}>
                {this.renderPlaces(places, maxLevel, selectableLevel)}
            </TreeSelect>
        )
    }
}

export default class extends Component {
    handleSelect = (value, node, extra) => {
        console.log('handleSelect', value, node, extra)
    }

    handleChange = (value, label, extra) => {
        console.log('handleChange', value, label, extra)
    }

    render() {
        return (
            <PlacesTreeSelect
              places={places}
              maxLevel={2}
              selectableLevel={2}
              style={{width: 300}}
              onSelect={this.handleSelect}
              onChange={this.handleChange}
              multiple={true}
              maxTagCount={2}
            />
        )
    }
}
