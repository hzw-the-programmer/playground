import React, { Component } from 'react'
import { Tree } from 'antd'

import { places } from './places'

class PlacesTree extends Component {
    renderPlaces(places, maxLevel) {
        if (places && places[0].level <= maxLevel)
            return places.map(place => (
                <Tree.TreeNode key={place.id} title={place.name} dataRef={place}>
                    {this.renderPlaces(place.children, maxLevel)}
                </Tree.TreeNode>
            ))
    }

    render() {
        const {places, maxLevel, ...props} = this.props

        return (
            <Tree {...props}>
                {this.renderPlaces(places, maxLevel)}
            </Tree>
        )
    }
}

export default class extends Component {
    handleExpand = expandedKeys => {
        console.log('expandedKeys', expandedKeys)
    }

    handleSelect = selectedKeys => {
        console.log('selectedKeys', selectedKeys)
    }

    render() {
        return (
            <PlacesTree
              places={places}
              maxLevel={3}
              onExpand={this.handleExpand}
              onSelect={this.handleSelect}
            />
        )
    }
}
