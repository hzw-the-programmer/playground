const REQUEST = 'REQUEST'
const SUCCESS = 'SUCCESS'
const FAILURE = 'FAILURE'

function createRequestTypes(base) {
  return [REQUEST, SUCCESS, FAILURE].reduce((acc, type) => {
    acc[type] = `${base}_${type}`
    return acc
  }, {})
}

const USER = createRequestTypes('USER')
console.log(USER.REQUEST)
console.log(USER.SUCCESS)
console.log(USER.FAILURE)