import { action } from './utils'

export const INCREMENT = 'INCREMENT'
export const DECREMENT = 'DECREMENT'
export const INCREMENT_ASYNC = 'INCREMENT_ASYNC'

export const increment = () => action(INCREMENT)
export const decrement = () => action(DECREMENT)
export const incrementAsync = () => action(INCREMENT_ASYNC)
