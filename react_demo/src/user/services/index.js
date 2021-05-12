export const fetchUser = id =>
  new Promise((resolve, reject) => {
    setTimeout(() => {
      const r = Math.random()
      console.log(r)
      if (r > 0.5)
        resolve({id: 1, name: 'Zhiwen He', age: 31})
      else
        reject('fetch error')
    }, 2000)
  })