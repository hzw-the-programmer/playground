requirejs.config({
    baseUrl: 'js/lib',
    paths: {
        ko: 'knockout-3.5.0',
        jquery: 'jquery-3.4.1.min',
        app: '../app'
    }
})

requirejs(['app/main'])
