import configureStore from './store/configureStore'
import rootSaga from './sagas'
// import { loadMoreStarred } from './actions'
// import { loadMoreStargazers } from './actions'
// import { loadUserPage } from './actions'
import { loadRepoPage } from './actions'

const store = configureStore()
store.runSaga(rootSaga)

// store.dispatch(loadMoreStarred('hzw-the-programmer'))
// store.dispatch(loadMoreStargazers('facebook/react'))
// setTimeout(
//   () => store.dispatch(loadMoreStargazers('facebook/react')),
//   5000
// )
// store.dispatch(loadUserPage('hzw-the-programmer'))
store.dispatch(loadRepoPage('hzw-the-programmer/python-play-ground'))
