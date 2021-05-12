import { normalize, schema } from 'normalizr'
import { camelizeKeys } from 'humps'

function getNextPageUrl(response) {
  const link = response.headers.get('link')
  if (!link) return null

  const nextLink = link.split(',').find(s => s.indexOf('rel="next"') > -1)
  if (!nextLink) return null

  return nextLink.split(';')[0].slice(1, -1)
}

const API_ROOT = 'https://api.github.com/'

function callApi(endpoint, schema) {
  const fullUrl = (endpoint.indexOf(API_ROOT) === -1) ? API_ROOT + endpoint : endpoint

  return fetch(fullUrl)
    .then(response =>
      response.json().then(json => ({ json, response }))
    ).then(({ json, response }) => {
      if (!response.ok) {
        return Promise.reject(json)
      }

      const camelizedJson = camelizeKeys(json)
      const nextPageUrl = getNextPageUrl(response)

      console.log(json)
      console.log(camelizedJson)
      console.log(normalize(camelizedJson, schema))

      return Object.assign({},
        normalize(camelizedJson, schema),
        { nextPageUrl })
    })
    .then(
      response => ({ response }),
      error => ({ error: error.message || 'Something bad happended' })
    )
}

const userSchema = new schema.Entity('users', {}, {
  idAttribute: 'login'
})

const repoSchema = new schema.Entity('repos', {}, {
  idAttribute: 'fullName'
})

repoSchema.define({
  owner: userSchema
})

const repoSchemaArray = [ repoSchema ]
const userSchemaArray = [ userSchema ]

export const fetchUser = login => callApi(`users/${login}`, userSchema)
export const fetchRepo = fullName => callApi(`repos/${fullName}`, repoSchema)
export const fetchStarred = url => callApi(url, repoSchemaArray)
export const fetchStargazers = url => callApi(url, userSchemaArray)
