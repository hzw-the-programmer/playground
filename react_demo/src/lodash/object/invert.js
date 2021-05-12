import _ from 'lodash'

const a = {'a': 1, 'b': 2, 'c': 1}
console.log(_.invert(a))
console.log(_.invertBy(a))
console.log(a)
