function myClass() {
}

myClass.prototype.x = {a: 1}

const a = new myClass();
const b = new myClass();

console.log(a.x, b.x, myClass.prototype.x)
console.log(a.x === myClass.prototype.x)
console.log(b.x === myClass.prototype.x)

a.x = {a: 1}

console.log(a.x, b.x, myClass.prototype.x)
console.log(a.x === myClass.prototype.x)
console.log(b.x === myClass.prototype.x)
