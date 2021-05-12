import { schema, normalize } from 'normalizr'

const API_ROOT = 'http://localhost/api/'

function callApi(endpoint, schema) {
  const url = (endpoint.indexOf(API_ROOT) === -1) ? API_ROOT + endpoint : endpoint

  return fetch(url)
    .then(response =>
      response.json().then(json => ({json, response})))
    .then(({json, response}) => {
      if (!response.ok) {
        return Promies.reject(json)
      }

      return {...normalize(json, schema)}
    })
    .then(
      response => ({response}),
      error => ({error: error.message || 'Something bad happened'})
    )
}

const placeSchema = new schema.Entity('places')
placeSchema.define({
  children: [ placeSchema ]
})

export const fetchPlace = id => callApi(`places.php?id=${id}`, placeSchema)
