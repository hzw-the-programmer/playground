var gPlacesCtrl = new PlacesCtrl({
  plant: {onChange: onChange},
  workshop: {onChange: onChange},
  region: {onChange: onChange},
  line: {onChange: onChange},
  station: {onChange: onChange},
});
var gTimer;
var gChannelsStatus;
window.addEventListener("message", receiveMessage, false);

$(document).ready(function () {
  getRealtimeStatus();
  gPlacesCtrl.init();
  $.getJSON("ajax/get_places.php", function(places) {
    gPlacesCtrl.setPlaces(places);
  });
});

function receiveMessage(event) {
  if (event.data === 'onUnselect') {
    clearTimeout(gTimer);
  } else {
    getRealtimeStatus();
  }
}

var jqXHR1;
function onChange(newVal, oldVal) {
  if (jqXHR1) {
    jqXHR1.abort();
  }

  const leaf = this.getLeaf();

  if (!leaf) {
    $("#content").empty();
  } else {
    if (leaf.level == 5) {
      jqXHR1 = $.getJSON('ajax/get_channels_under_station.php?id=' + leaf.id, function(devices) {
        drawDevices(devices);
      });
    } else {
      jqXHR1 = $.getJSON('ajax/get_child_places.php?pid=' + leaf.id, function(places) {
        if (leaf.level == 4) {
          jqXHR1 = $.getJSON('ajax/get_equipments.php?pid=' + leaf.id, function(equipments) {
            draw(places);
            drawEquipments(equipments);
          });
        } else {
          draw(places);
        }
      });
    }
  }
}

function getColors() {
  return ['#480091', '#FF0C00', '#EF8B00', '#64FF00', '#BCBCBC', '#000000'];
}

function getColorPriority(color) {
  for (let i = 0; i < getColors().length; i++) {
    if (color === getColors()[i]) {
      return i;
    }
  }
}

function getChannelColor(channel) {
  var type = channel.type;
  var status = channel.status;

  var color = getColors()[5];

  if (status < 256) {
    switch(type) {
      case 'GND-H':
      case 'GND-L':
        if (status & 0x20) {
          color = getColors()[1];
        } else {
          color = getColors()[3];
        }
        break;

      case 'WS':
        if (status & 0x80) {
          color = getColors()[0];
        } else if (!(status & 0x10) && (status & 0x40)) {
          color = getColors()[2];
        } else if (status & 0x40) {
          color = getColors()[4];
        } else if (status & 0x20) {
          color = getColors()[1];
        } else {
          color = getColors()[3];
        }
        break;

      case 'HUMI':
      case 'TEMP':
      case 'VB':
        if (status & 0x40) {
          color = getColors()[4];
        } else if (status & 0x20) {
          color = getColors()[1];
        } else {
          color = getColors()[3];
        }
        break;
    }
  }

  return color;
}

function getPlaceColor(place, channels) {
  var color = getColors()[5];

  for (let channel of channels) {
    if (channel['ancestors'][place.level] === place.id) {
      var c = getChannelColor(channel);
      var p1 = getColorPriority(c);
      var p2 = getColorPriority(color);
      if (p1 < p2) {
        color = c;
      }
    }
  }

  return color;
}

function updateStatus(channels) {
  $('.indicator').each(function() {
    var place = this.place;
    var color = getPlaceColor(place, channels);
    this.style.background = color;
  })
}

function getRealtimeStatus() {
  $.getJSON('ajax/get_realtime_status.php', function(channelsStatus) {
    gChannelsStatus = channelsStatus;
    updateStatus(channelsStatus);
    gTimer = setTimeout(getRealtimeStatus, 1000);
  });
}

function onClick() {
  gPlacesCtrl.setLeaf(this.place);
}

function makeIndicator(place) {
  var div = document.createElement('div');
  div.style.position = 'absolute';
  div.style.left = place.left + 'px';
  div.style.top = place.top + 'px';
  div.style.width = '30px';
  div.style.height = '30px';
  div.style.border = '5px solid #000000';
  div.style.borderRadius = '20px';
  div.style.background = getPlaceColor(place, gChannelsStatus);
  div.setAttribute('class', 'indicator');
  div.place = place;
  div.onclick = onClick;
  $('#content').append(div);
}

function drawWorkshop(places) {
  for (let place of places) {
    makeIndicator(place);

    var span = document.createElement('span');
    span.style.position = 'absolute';
    span.style.left = (parseInt(place.left) + 60) + 'px';
    span.style.top = place.top + 'px';
    span.innerText = place.name;
    span.style.fontSize = '1.5em';
    span.style.fontWeight = 'bold';
    span.style.lineHeight = '40px';
    $('#content').append(span);

    var img = document.createElement('img');
    img.style.position = 'absolute';
    img.style.left = place.left + 'px';
    img.style.top = (parseInt(place.top) + 40) + 'px';
    img.src = 'image/region.jpg';
    $('#content').append(img);
  }
}

function drawLine(places) {
  for (let place of places) {
    var div = document.createElement("div");
    div.style.position = "absolute";
    div.style.left = place.left + "px";
    div.style.top = place.top + "px";
    div.style.width = "30px";
    div.style.height = "30px";
    div.style.border = "5px solid #000000";
    div.style.borderRadius = "20px";
    div.style.background = getPlaceColor(place, gChannelsStatus);
    div.setAttribute ('class', "indicator");
    div.place = place;
    div.onclick = onClick;
    $('#content').append(div);

    var span = document.createElement("span");
    span.style.position = "absolute";
    span.style.left = (parseInt(place.left) + 5) + "px";
    span.style.top = (parseInt(place.top) - 38) + "px";
    span.innerText = place.name;
    span.style.fontSize="1.5em";
    span.style.fontWeight="bold";
    span.style.lineHeight = "40px";
    $('#content').append(span);

    var img = document.createElement("img");
    img.style.position = "absolute";
    img.style.left = (parseInt(place.left) + 60) + "px";
    img.style.top = place.top + "px";
    img.src = "image/line.jpg";
    $('#content').append(img);
  }
}

function drawEquipments(equipments) {
  for (let equipment of equipments) {
    var img = document.createElement("img");
    img.style.position = "absolute";
    img.style.left = equipment.left + "px";
    img.style.top = equipment.top + "px";
    img.src = "image/station/" + equipment.name + ".jpg";
    $('#content').append(img);
  }
}

function drawStation(places) {
  for (let place of places) {
    var div = document.createElement('div');
    div.style.position = 'absolute';
    div.style.left = place.left + 'px';
    div.style.top = place.top + 'px';
    div.style.width = '30px';
    div.style.height = '30px';
    div.style.border = '5px solid #000000';
    div.style.borderRadius = '20px';
    div.style.background = getPlaceColor(place, gChannelsStatus);
    div.style.zIndex = '10';
    div.setAttribute ('class', "indicator");
    div.place = place;
    div.onclick = onClick;
    $('#content').append(div);

    var span = document.createElement('span');
    span.style.position = 'absolute';
    span.style.left = (parseInt(place.left) + 5) + 'px';
    span.style.top = (parseInt(place.top) - 38) + 'px';
    span.innerText = place.name;
    span.style.fontSize = '1em';
    span.style.fontWeight = 'bold';
    span.style.lineHeight = '40px';
    span.style.zIndex = '10';
    $('#content').append(span);
  }
}

function drawDeviceBg(device, left, top) {
  var img, span;

  // background
  img = document.createElement("img");
  img.style.position = "absolute";
  img.style.left = left + "px";
  img.style.top = top + "px";
  img.src = "image/background.jpg";
  $("#content").append(img);

  left += 40;
  top += 10;
  span = document.createElement("span");
  span.style.position = "absolute";
  span.style.left = left + "px";
  span.style.top = top + "px";
  span.innerText = "DAT SN: " + device.sn;
  span.style.fontSize = "1.5em";
  span.style.fontWeight = "bold";
  $("#content").append(span);

  // WS
  left += 10;
  top += 40
  img = document.createElement("img");
  img.style.position = "absolute";
  img.style.left = left + "px";
  img.style.top = top + "px";
  img.src = "image/WS.jpg";
  $("#content").append(img);

  span = document.createElement("span");
  span.style.position = "absolute";
  span.style.left = (left + 75 + 10) + "px";
  span.style.top = (top + 10) + "px";
  span.innerText = "WS:";
  span.style.fontWeight = "bold";
  $("#content").append(span);

  // GND-L
  top += 58 + 15;
  img = document.createElement("img");
  img.style.position = "absolute";
  img.style.left = left + "px";
  img.style.top = top + "px";
  img.src = "image/GND-L.jpg";
  $("#content").append(img);

  span = document.createElement("span");
  span.style.position = "absolute";
  span.style.left = (left + 75 + 10) + "px";
  span.style.top = top + 10 + "px";
  span.innerText = "GND-L:";
  span.style.fontWeight = "bold";
  $("#content").append(span);

  // GND-H
  top += 75 + 15;
  img = document.createElement("img");
  img.style.position = "absolute";
  img.style.left = left + "px";
  img.style.top = top + "px";
  img.src = "image/GND-H.jpg";
  $("#content").append(img);

  span = document.createElement("span");
  span.style.position = "absolute";
  span.style.left = (left + 75 + 10) + "px";
  span.style.top = (top + 10) + "px";
  span.innerText = "GND-H:";
  span.style.fontWeight="bold";
  $("#content").append(span);

  // VB
  top += 62 + 15;
  img = document.createElement("img");
  img.style.position = "absolute";
  img.style.left = left + "px";
  img.style.top = top + "px";
  img.src = "image/VB.jpg";
  $("#content").append(img);

  span = document.createElement("span");
  span.style.position = "absolute";
  span.style.left = left + 75 + 10 + "px";
  span.style.top = top + 10 + "px";
  span.innerText = "VB:";
  span.style.fontWeight="bold";
  $("#content").append(span);

  // TEMP
  top += 65 + 15;
  img = document.createElement("img");
  img.style.position = "absolute";
  img.style.left = left + "px";
  img.style.top = top + "px";
  img.src = "image/temp.jpg";
  $("#content").append(img);

  span = document.createElement("span");
  span.style.position = "absolute";
  span.style.left = left + 75 + 10 + "px";
  span.style.top = top + 10 + "px";
  span.innerText = "TEMP:";
  span.style.fontWeight="bold";
  $("#content").append(span);

  // HUM
  top += 65 + 15;
  img = document.createElement("img");
  img.style.position = "absolute";
  img.style.left = left + "px";
  img.style.top = top + "px";
  img.src = "image/hum.jpg";
  $("#content").append(img);

  span = document.createElement("span");
  span.style.position = "absolute";
  span.style.left = left + 75 + 10 + "px";
  span.style.top = top + 10 + "px";
  span.innerText = "HUM:";
  span.style.fontWeight="bold";
  $("#content").append(span);
}

function drawChannel(channel) {
  var place = channel.place;
  var coordinate = place.coordinate;
  var div;

  div = document.createElement('div');
  div.style.position = 'absolute';
  div.style.left = coordinate.left + 'px';
  div.style.top = coordinate.top + 'px';
  div.style.width = 25 + 'px';
  div.style.height = 25 + 'px';
  div.style.background = getPlaceColor(place, gChannelsStatus);
  div.style.border = '3px solid #000000';
  div.setAttribute ('class', "indicator");
  div.place = place;
  div.onmouseover = mouseover;
  div.onmouseout = mouseout;

  if (channel.type == 'GND-H') {
      div.style.borderRadius = '6px';
  } else if (channel.type == 'WS') {
      div.style.borderRadius = (coordinate.right - coordinate.left) / 2 + 'px';
  } else if (channel.type == 'VB') {
      div.style.transform = 'rotate(45deg)';
  }

  $("#content").append(div);
}

function drawDevice(device, left, top) {
  drawDeviceBg(device, left, top);
  for (let channel of device.channels) {
    drawChannel(channel);
  }
}

function drawDevices(devices) {
  var left, top;

  $("#content").empty();
  devices.forEach(function(device, i) {
    left = 30;
    top = 50 + 50 + (i * (561 + 50) );
    drawDevice(device, left, top);
  });
}

function draw(places) {
  $('#content').empty();
  if (!places || places.length === 0) return;
  var place = places[0];
  switch(place.level) {
    case 2:
    case 3:
      drawWorkshop(places);
      break;
    case 4:
      drawLine(places);
      break;
    case 5:
      drawStation(places);
      break;
  }
}

var jqXHR2;
function mouseover() {
  $("#name").html("名称: ");
  $("#slot").html("列号: ");
  $("#port").html("端口: ");
  for (let i = 0; i < 5; i++) {
    $("#time" + i).html("");
    $("#status" + i).html("");
  }

  $("#msg").css("left", parseInt(this.style.left) + parseInt(this.style.width));
  $("#msg").css("top", parseInt(this.style.top) + parseInt(this.style.height));
  $("#msg").show();

  const place = this.place;
  jqXHR2 = $.getJSON("ajax/get_latest_channel_status.php?pid=" + place.id, function(cs) {
    $("#name").html($("#name").html() + cs[0].name);
    $("#slot").html($("#slot").html() + cs[0].slot);
    $("#port").html($("#port").html() + cs[0].port);

    for (let i = 0; i < 5; i++) {
      $("#time" + i).html(cs[i].ctime);
      $("#status" + i).html(cs[i].status);
    }
  });
}

function mouseout() {
  if (jqXHR2) {
    jqXHR2.abort();
  }
  $("#msg").hide();
}
