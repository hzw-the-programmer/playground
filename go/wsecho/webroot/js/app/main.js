define(
    ['ko', 'app/MessageList'],
    function(ko, MessageList) {
        var ws = new WebSocket("ws://localhost:8080/echo")
        var ml = new MessageList(ws)
        ko.applyBindings(ml)
    }
)
