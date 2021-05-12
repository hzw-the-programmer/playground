import React from 'react'
import './Container.css'
// import { Control, Description } from './card/Grid'
// import { Control, Description } from './card/Inner'
// import { Control, Description } from './list/Basic'
// import { Control, Description } from './list/Grid'
import { Control, Description } from './grid/Basic'

const Container = () => (
  <section className="code-box">
    <section className="code-box-demo">
      <Control />
    </section>
    <section className="code-box-meta">
      <Description />
    </section>
  </section>
)

export default Container