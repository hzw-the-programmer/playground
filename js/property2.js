const o = {}
const descriptor = {value: 1, writable: true}
Object.defineProperty(o, 'x', descriptor)

let ownDescriptor1 = Object.getOwnPropertyDescriptor(o, 'x')
let ownDescriptor2 = Object.getOwnPropertyDescriptor(o, 'x')
console.log(ownDescriptor1, ownDescriptor2, descriptor)
console.log(ownDescriptor1 === ownDescriptor2)

o.x = 2
console.log(ownDescriptor1, ownDescriptor2, descriptor)

ownDescriptor = Object.getOwnPropertyDescriptor(o, 'x')
console.log(ownDescriptor, descriptor)
