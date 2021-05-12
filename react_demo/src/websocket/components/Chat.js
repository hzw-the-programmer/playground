/**
 * Author: Zhiwen He <18676797056@163.com>
 */
import React from 'react'
import { connect } from "react-redux";

const Chat = ({onSend}) =>
  <div>
    <input type="text" />
    <button onClick={onSend}>
      Send
    </button>
  </div>

function mapStateToProps(state) {
  return {}
}

function mapDispatchToProps(dispatch) {
  return {
    onSend: () => dispatch({type: 'SEND', payload: 'I love you!!!'})
  }
}

export default connect(mapStateToProps, mapDispatchToProps)(Chat)
