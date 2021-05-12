/**
 * Author: Zhiwen He <18676797056@163.com>
 */
import { actionChannel, all, call, cancel, cancelled, fork, put, take } from 'redux-saga/effects'
import { delay, eventChannel, END } from 'redux-saga'

function* delayAndIncrement() {
  yield delay(5000)
  yield put({type: 'INCREMENT'})
}

function* watchIncrementAsync() {
  const channel = yield actionChannel('INCREMENT_ASYNC')
  while (true) {
    const action = yield take(channel)
    yield call(delayAndIncrement)
  }
  /*
  while (true) {
    const action = yield take('INCREMENT_ASYNC')
    //yield call(delayAndIncrement)
    yield fork(delayAndIncrement)
  }
  */
}

function* countdown(num) {
  return eventChannel(emitter => {
    const iv = setInterval(() => {
      num--;
      if (num < 0) {
        emitter(END)
      } else {
        console.log('emitter')
        emitter(num)
      }
    }, 1000)

    return () => {
      console.log('clear')
      clearInterval(iv)
    }
  })
}

function* watchCountdown() {
  const channel = yield call(countdown, 10)

  try {
    while (true) {
      console.log('begin take')
      const num = yield take(channel)
      console.log(num)
    }
  } finally {
    if (yield cancelled()) {
      console.log('cancelled')
      channel.close()
    }
    console.log('end')
  }
  console.log('end1')
}

function* cancelTask() {
  console.log('cancelTask before delay')
  //yield delay(5000)
  try {
    yield delay(5000)
  } finally {
    console.log('cancelTask in finally 1')
    yield delay(1000)
    console.log('cancelTask in finally 2')
  }
  console.log('cancelTask after delay')
}

function* testCancelTask() {
  const task = yield fork(cancelTask)
  yield delay(2000)
  console.log('before cancel')
  yield cancel(task)
  console.log('after cancel')
}

function* testCancelTask1() {
  const task = yield fork(watchCountdown)
  console.log('after fork')
  yield delay(2000)
  console.log('before cancel')
  yield cancel(task)
  console.log('after cancel')
}

function* watchSend(achan, ws) {
  while (true) {
    const {payload} = yield take(achan)
    ws.send(payload)
  }
}

function* watchWebSocket() {
  console.log('watchWebSocket enter')

  const ws = new WebSocket("ws://echo.websocket.org/")
  const wschan = eventChannel(emitter => {
    ws.onopen = evt => {
      console.log('onopen')
      emitter({type: 'ONOPEN', payload: evt})
    }
    ws.onmessage = evt => {
      console.log('onmessage')
      emitter({type: 'ONMESSAGE', payload: evt})
    }
    ws.onclose = evt => {
      console.log('onclose')
      emitter({type: 'ONCLOSE', payload: evt})
    }
    ws.onerror = evt => {
      console.log('onerror')
      emitter({type: 'ONERROR', payload: evt})
    }

    return () => {

    }
  })

  const achan = yield actionChannel('SEND')

  while (true) {
    const {type, payload} = yield take(wschan)
    switch (type) {
      case 'ONOPEN':
      yield fork(watchSend, achan, ws)
      break;
      case 'ONMESSAGE':
      console.log(payload)
      break;
    }
  }

  console.log('watchWebSocket exit')
}

export default function* rootSaga() {
  console.log('rootSaga enter')
  yield all([
    fork(watchIncrementAsync),
    //fork(watchCountdown),
    //fork(testCancelTask),
    //fork(testCancelTask1),
    fork(watchWebSocket),
  ])
  console.log('rootSaga exit')
}
