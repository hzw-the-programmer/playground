define(
    ['ko', 'jquery', 'app/Message'],
    function(ko, $, Message) {
        function MessageList(ws) {
            var that = this;
            this.messages = ko.observableArray()

            this.editingMessage = ko.observable(new Message())

            this.send = function() {
                var model = this.editingMessage().toModel()
                ws.send(JSON.stringify(model))
                var msg = new Message()
                msg.author(model.author)
                this.editingMessage(msg)
            }

            ws.onmessage = function(e) {
                model = JSON.parse(e.data)
                var msg = new Message(model)
                that.messages.push(msg)
            }
        }

        return MessageList
    }
)
