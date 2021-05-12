import React from 'react'

const CWIDTH = 25
const CHEIGHT = 25
const CHGAP = 5
const CVGAP = 5
const CROWS = 6
const CCOLUMNS = 4
const TGAP = 4
const THEIGHT = 25

const Station = (props) => {
  let paddingLeft, paddingTop, paddingRight, paddingBottom
  paddingLeft = props.paddingLeft
  if (!paddingLeft) {
    paddingLeft = props.padding
  }
  paddingTop = props.paddingTop
  if (!paddingTop) {
    paddingTop = props.padding
  }
  paddingRight = props.paddingRight
  if (!paddingRight) {
    paddingRight = props.padding
  }
  paddingBottom = props.paddingBottom
  if (!paddingBottom) {
    paddingBottom = props.padding
  }
  if (!paddingLeft) {
    paddingLeft = 0
  }
  if (!paddingTop) {
    paddingTop = 0
  }
  if (!paddingRight) {
    paddingRight = 0
  }
  if (!paddingBottom) {
    paddingBottom = 0
  }

  // console.log(paddingLeft, paddingTop, paddingRight, paddingBottom)

  const cs = props.channels.map((channel, index) => {
    const x = paddingLeft + channel.column * (CWIDTH + CHGAP)
    const y = paddingTop + channel.row * (CHEIGHT + CVGAP)
    const attr = {
      x: x,
      y: y,
      cx: x + CWIDTH / 2,
      cy: y + CHEIGHT / 2,
      r: CWIDTH / 2,
      fill: channel.color,
      key: channel.id,
      width: CWIDTH,
      height: CHEIGHT,
      onClick: () => props.onChannelClick(channel),
      onMouseEnter: () => props.onChannelMouseEnter(channel),
      onMouseLeave: () => props.onChannelMouseLeave(channel)
    }

    switch (channel.type) {
      case 3:
        return <circle {...attr} />
      default:
        return <rect {...attr} />
    }
  })

  const tbx = paddingLeft
  const tby = paddingTop + CHEIGHT * CROWS + CVGAP * (CROWS - 1) + TGAP
  const cwidth = CWIDTH * CCOLUMNS + CHGAP * (CCOLUMNS - 1)
  const width = paddingLeft + cwidth + paddingRight
  const height = tby + THEIGHT + paddingBottom
  const tx = width / 2
  const ty = tby + THEIGHT / 2

  return (
    <svg style={{border: '1px solid black'}} width={width} height={height}>
      {cs}
      <rect x={tbx} y={tby} width={cwidth} height={THEIGHT} fill="none" stroke="black" />
      <text x={tx} y={ty} textAnchor="middle" alignmentBaseline="middle">
        Station 1
      </text>
    </svg>
  )
}

export default Station
