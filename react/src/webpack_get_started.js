function component() {
  var elem = document.createElement('div');
  elem.innerHTML = _.join(['Hello', 'Zhiwen He'], ' ');
  return elem;
}

document.body.append(component());
