import React, {Component} from 'react'

import 'ol/ol.css';
import Map from 'ol/Map';
import View from 'ol/View';
import TileLayer from 'ol/layer/Tile';
import {OSM, TileDebug} from 'ol/source';

class Map1 extends Component {
    constructor(props) {
        super(props)
        this.map = React.createRef();
    }

    render() {
        return <div ref={this.map} style={{width: '100%', height: '400px'}}></div>
    }

    componentDidMount() {
      var map = new Map({
        layers: [
          new TileLayer({
            source: new OSM()
          }),
          new TileLayer({
            source: new TileDebug()
          })
        ],
        target: this.map.current,
        view: new View({
          center: [0, 0],
          zoom: 1
        })
      });
  
      // save map and layer references to local state
      this.setState({ 
        map: map,
      });
    }
}

export default Map1
