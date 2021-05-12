import { schema, normalize } from 'normalizr'

const place = new schema.Entity('places')
place.define({
  children: [ place ]
})

fetch('http://localhost/api/get_places.php')
  .then(response => response.json())
  .then(json => {
    console.log(normalize(json, [ place ]))
  })