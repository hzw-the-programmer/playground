let knightPosition = [7, 1]
let observer = null

function emitChange() {
  observer(knightPosition)
}

function canMoveKnight(row, column) {
  const [r, c] = knightPosition
  const dr = row - r
  const dc = column - c

  return (Math.abs(dr) === 1 && Math.abs(dc) === 2) ||
         (Math.abs(dr) === 2 && Math.abs(dc) === 1)
}

export function observe(o) {
  if (observer) {
    throw new Error('Multiple observers not implemented.')
  }

  observer = o
  emitChange()
}

export function moveKnight(row, column) {
  if (canMoveKnight(row, column)) {
    knightPosition = [row, column]
    emitChange()
  }
}
