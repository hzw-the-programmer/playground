requirejs.config({
    paths: {
        ko: '../vendor/knockout-3.5.0'
    }
})

requirejs(['ko'], function(ko) {
    var person = {
        name: ko.observable("hzw"),
        age: ko.observable(32)
    }
    ko.applyBindings(person)
})
