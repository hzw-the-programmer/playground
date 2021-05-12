/**
 * Author: Zhiwen He <18676797056@163.com>
 */
export default function rootReducer(state = 0, action) {
  switch (action.type) {
    case 'INCREMENT':
      return state + 1
    case 'DECREMENT':
      return state - 1
    default:
      return state;
  }
}
