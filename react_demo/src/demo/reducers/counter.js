import { INCREMENT, DECREMENT } from '../actions/counter'

export default function counter(state = {value: 0}, action) {
    switch (action.type) {
        case INCREMENT:
            return {value: state.value + 1}
        case DECREMENT:
            return {value: state.value - 1}
        default:
            return state
    }
}
