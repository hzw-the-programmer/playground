function PlacesCtrl(ctrls) {
  this.ctrls = ctrls;
}

PlacesCtrl.prototype.init = function() {
  for (var key in this.ctrls) {
    var ctrl = this.ctrls[key];

    var id = ctrl.id;
    if (id === undefined) {
      id = this.defaultProps[key].id;
      ctrl.id = id;
    }

    var onChange = this.defaultProps[key].onChange;
    onChange = onChange.bind(this);

    var label = ctrl.label ? ctrl.label : this.defaultProps[key].label;
    var labelWidth = ctrl.labelWidth ? ctrl.labelWidth : this.defaultProps[key].labelWidth;
    var width = ctrl.width ? ctrl.width : this.defaultProps[key].width;

    $(id).combobox({
      label: label,
      labelWidth: labelWidth,
      width: width,
      textField: 'name',
      valueField: 'id',
      onChange: onChange
    });
  }
};

PlacesCtrl.prototype.getParentKey = function(key) {
  switch (key) {
    case 'plant':
      return 'root';
    case 'workshop':
      return 'plant';
    case 'region':
      return 'workshop';
    case 'line':
      return 'region';
    case 'station':
      return 'line';
    case 'channel':
      return 'station';
  }
};

PlacesCtrl.prototype.levelToKey = function(level) {
  switch (level) {
    case 0:
      return 'root';
    case 1:
      return 'plant';
    case 2:
      return 'workshop';
    case 3:
      return 'region';
    case 4:
      return 'line';
    case 5:
      return 'station';
    case 6:
      return 'channel';
  }
};

PlacesCtrl.prototype.load = function(key) {
  if (key in this.ctrls) {
    var ctrl = this.ctrls[key];
    var place = this.getCurrent(this.getParentKey(key));
    var data = place && place.children ? place.children.slice(0) : [];
    ctrl.place = null;
    if (data.length != 0)
      data.unshift({id: '', name: '*'});
    $(ctrl.id).combobox('loadData', data);
    $(ctrl.id).combobox('clear');
  }
};

PlacesCtrl.prototype.findChildIn = function(place, id) {
  if (place) {
    for (let child of place.children) {
      if (child['id'] == id) {
        return child;
      }
    }
  }
}

PlacesCtrl.prototype.getCurrent = function(key) {
  if (key in this.ctrls) {
    return this.ctrls[key].place;
  } else if (key === 'root') {
    return this.places[0];
  }
};

PlacesCtrl.prototype.setCurrent = function(key, place) {
  if (key in this.ctrls) {
    this.ctrls[key].place = place;
  }
};

PlacesCtrl.prototype.setPlaces = function(places) {
  this.places = places;
  this.load('plant');
};

PlacesCtrl.prototype.getLeaf = function() {
  var keys = ['channel', 'station', 'line',
              'region', 'workshop', 'plant'];
  for (let key of keys) {
    var place = this.getCurrent(key);
    if (place)
      return place;
  }
};

PlacesCtrl.prototype.setLeaf = function(place) {
  var key = this.levelToKey(place.level);
  if (key in this.ctrls) {
    var ctrl = this.ctrls[key];
    $(ctrl.id).combobox('select', place.id);
  }
};

PlacesCtrl.prototype.onChange = function(key, newVal, oldVal) {
  if (key in this.ctrls) {
    var onChange = this.ctrls[key].onChange;
    if (onChange)
      onChange.call(this, newVal, oldVal);
  }
};

PlacesCtrl.prototype.onPlantChange = function(newVal, oldVal) {
  this.setCurrent('plant', this.findChildIn(this.getCurrent('root'), newVal));
  this.load('workshop');
  this.onChange('plant', newVal, oldVal);
};

PlacesCtrl.prototype.onWorkshopChange = function(newVal, oldVal) {
  this.setCurrent('workshop', this.findChildIn(this.getCurrent('plant'), newVal));
  this.load('region');
  this.onChange('workshop', newVal, oldVal);
};

PlacesCtrl.prototype.onRegionChange = function(newVal, oldVal) {
  this.setCurrent('region', this.findChildIn(this.getCurrent('workshop'), newVal));
  this.load('line');
  this.onChange('region', newVal, oldVal);
};

PlacesCtrl.prototype.onLineChange = function(newVal, oldVal) {
  this.setCurrent('line', this.findChildIn(this.getCurrent('region'), newVal));
  this.load('station');
  this.onChange('line', newVal, oldVal);
};

PlacesCtrl.prototype.onStationChange = function(newVal, oldVal) {
  this.setCurrent('station', this.findChildIn(this.getCurrent('line'), newVal));
  this.load('channel');
  this.onChange('station', newVal, oldVal);
};

PlacesCtrl.prototype.onChannelChange = function(newVal, oldVal) {
  this.setCurrent('channel', this.findChildIn(this.getCurrent('station'), newVal));
  this.onChange('channel', newVal, oldVal);
};

PlacesCtrl.prototype.defaultProps = {
  plant: {
    id: '#plant',
    onChange: PlacesCtrl.prototype.onPlantChange,
    label: '工厂:',
    labelWidth: 60,
    width: 200,
  },
  workshop: {
    id: '#workshop',
    onChange: PlacesCtrl.prototype.onWorkshopChange,
    label: '车间:',
    labelWidth: 60,
    width: 200,
  },
  region: {
    id: '#region',
    onChange: PlacesCtrl.prototype.onRegionChange,
    label: '区域:',
    labelWidth: 60,
    width: 200,
  },
  line: {
    id: '#line',
    onChange: PlacesCtrl.prototype.onLineChange,
    label: '线体:',
    labelWidth: 60,
    width: 200,
  },
  station: {
    id: '#station',
    onChange: PlacesCtrl.prototype.onStationChange,
    label: '工位:',
    labelWidth: 60,
    width: 200,
  },
  channel: {
    id: '#channel',
    onChange: PlacesCtrl.prototype.onChannelChange,
    label: '监控点:',
    labelWidth: 60,
    width: 200,
  },
};
