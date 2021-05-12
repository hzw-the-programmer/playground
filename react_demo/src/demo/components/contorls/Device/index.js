import React from 'react'

export default class Device extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            channels: [{
                slot: 2,
                port: 1,
                type: 0x0C
            }, {
                slot: 2,
                port: 0,
                type: 0x0C
            }, {
                slot: 1,
                port: 1,
                type: 0x08
            }, {
                slot: 1,
                port: 0,
                type: 0x08
            }]
        }
    }

    render() {
        const { channels } = this.state
        const minSlot = channels.reduce((acc, chan) => acc.slot < chan.slot ? acc : chan).slot
        const maxSlot = channels.reduce((acc, chan) => acc.slot > chan.slot ? acc : chan).slot
        const minPort = channels.reduce((acc, chan) => acc.port < chan.port ? acc : chan).port
        const maxPort = channels.reduce((acc, chan) => acc.port > chan.port ? acc : chan).port
        function getChannel(slot, port) {
            return channels.find(chan => chan.slot === slot && chan.port === port)
        }

        const rows = []
        for (let port = minPort; port <= maxPort; port++) {
            const columns = []
            for (let slot = minSlot; slot <= maxSlot; slot++) {
                const chan = getChannel(slot, port)
                columns.push((
                    <input key={slot} value={`${chan.slot} ${chan.port}`} />
                ))
            }
            rows.push((
                <div key={port}>
                    {columns}
                </div>
            ))
        }

        return (
            <div>
                {rows}
            </div>
        )
    }
}
