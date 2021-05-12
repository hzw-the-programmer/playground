const API_ROOT = 'http://127.0.0.1/api'

export const submitPriceForm = values => {
  return fetch(API_ROOT + '/process_price_form.php', {
    method: 'POST',
    headers: {
      'content-type': 'application/json'
    },
    body: JSON.stringify(values),
    //mode: 'no-cors'
  }).then(response =>
    response.json().then(json => ({json, response}))
  ).then(({json, response}) => {
    if (!response.ok) {
      return Promise.reject(json)
    }

    return json
  }).then(
    response => ({response}),
    error => ({error: error.message || 'Something bad happened'})
  )
}
