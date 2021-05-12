define(
    ['ko'],
    function(ko) {
        function Message(model) {
            if (model !== undefined) {
                this.author = ko.observable(model.author)
                this.message = ko.observable(model.message)
            } else {
                this.author = ko.observable('Anonymous')
                this.message = ko.observable('')
            }

            this.toModel = function() {
                return {
                    author: this.author(),
                    message: this.message()
                }
            }
        }

        return Message
    }
)
