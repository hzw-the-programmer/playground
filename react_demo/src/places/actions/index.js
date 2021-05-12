export const REQUEST = 'REQUEST'
export const SUCCESS = 'SUCCESS'
export const FAILURE = 'FAILURE'

const createRequestTypes = base =>
  [REQUEST, SUCCESS, FAILURE].reduce((acc, type) => {
    acc[type] = `${base}_${type}`
    return acc
  }, {})

export const PLACE = createRequestTypes('PLACE')
export const LOAD_PLACE = 'LOAD_PLACE'

const action = (type, payload) => ({type, ...payload})

export const place = {
  request: id => action(PLACE[REQUEST], {id}),
  success: (id, response) => action(PLACE[SUCCESS], {id, response}),
  failure: (id, error) => action(PLACE[FAILURE], {id, error})
}

export const loadPlace = id => action(LOAD_PLACE, { id })
