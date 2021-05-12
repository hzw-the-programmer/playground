function Person(fn, ln) {
    console.log(this.__proto__, Person.prototype, this.__proto__ == Person.prototype)
    this.fn = fn
    this.ln = ln
}

p = new Person('h', 'w')
