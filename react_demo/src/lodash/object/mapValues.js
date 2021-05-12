import _ from 'lodash'

const users = {
  'fred': {
    'user': 'fred',
    'age': 40
  },
  'pebbles': {
    'user': 'pebbles',
    'age': 1
  }
}

console.log(_.mapValues(users, user => user.age))

const a = {
  beta: {
    value: null,
    error: null
  },
  hamma: {
    value: null,
    error: null
  },
  zerta: {
    value: null,
    error: null
  },
  mozes: 5
}

const b = {
  beta: 5,
  hamma: 2
}

console.log(_.mapValues(b, value => ({value})))
console.log(b)

console.log(_.merge({}, a, _.mapValues(b, value => ({value}))))
console.log(a)

// console.log(_.merge(a, _.mapValues(b, value => ({value}))))
// console.log(a)