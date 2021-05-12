import { INCREMENT, DECREMENT } from '../actions'

const root = (state = 0, action) => {
  switch (action.type) {
    case INCREMENT:
      return state + 1
    case DECREMENT:
      return state - 1
    default:
      // console.log('reducer', action)
      return state
  }
}

export default root