import _ from 'lodash'

const object = {
  'a': [{'b': 2}, {'d': 4}, {'g': 7}]
}

const other = {
  'a': [{'c': 3}, {'e': 5}, {'f': 6}]
}

// const res = _.merge(object, other)
// console.log(res === object)
// console.log(object)

const res = _.merge({}, object, other)
console.log(res === object)
console.log(res)