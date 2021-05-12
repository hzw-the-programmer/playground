export function fetchBasicTable() {
  const dataSource = [{
    id: 1,
    name: '何之问',
    age: 31,
    address: '广东省深圳市福田区彩田路7006号'
  }]

  return new Promise((resolve, reject) => {
    setTimeout(() => {
      resolve({response: dataSource})
    }, 5000)
  })
}
