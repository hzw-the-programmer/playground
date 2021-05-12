import _ from 'lodash';
import './style.css';
import Icon from './calculator.png';
import Data from './data.xml';
import printMe from './print.js';

function component() {
  var elem = document.createElement('div');
  elem.innerHTML = _.join(['Hello', 'Zhiwen', 'He'], ' ');
  elem.classList.add('hello');

  var img = new Image();
  img.src = Icon;
  elem.appendChild(img);

  var btn = document.createElement('button');
  btn.innerHTML = 'Click me and check the console!';
  btn.onclick = printMe;
  elem.appendChild(btn);

  console.log(Data);

  return elem;
}

document.body.appendChild(component());