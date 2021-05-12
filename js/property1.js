function myClass() {
}

let value = {a: 1}
Object.defineProperty(myClass.prototype, 'x', {
  get() {return value},
  set(v) {value = v}
})
/*
Object.defineProperty(myClass.prototype, 'x', {
  get() {return this.value},
  set(v) {this.value = v}
})
*/

const a = new myClass();
const b = new myClass();

console.log(a.x, b.x, myClass.prototype.x)
console.log(a.x === myClass.prototype.x)
console.log(b.x === myClass.prototype.x)

a.x = {a: 2}

console.log(a.x, b.x, myClass.prototype.x)
console.log(a.x === myClass.prototype.x)
console.log(b.x === myClass.prototype.x)
