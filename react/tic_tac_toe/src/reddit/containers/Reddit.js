import React from 'react'
import { Provider } from 'react-redux'
import configureStore from '../configureStore'
import AsyncApp from './AsyncApp'

const store = configureStore()

const Reddit = () => (
  <Provider store={store}>
    <AsyncApp />
  </Provider>
)

export default Reddit
