const Koa = require('koa')
const app = new Koa()

app.use(async (ctx, next) => {
    await next()
    const rt = ctx.response.get('X-Response-Time')
    console.log(`${ctx.method} ${ctx.url} - ${rt}`)
})

app.use(async (ctx, next) => {
    const start = Date.now()
    await next()
    const ms = Date.now() - start
    ctx.set('X-Response-Time', `${ms}ms`)
})

app.use(async ctx => {
    ctx.body = 'Hello World'
})

//app.listen(3000)

const http = require('http')

http.createServer(app.callback()).listen(3000)

const https = require('https')

const fs = require('fs')
const options = {
    key: fs.readFileSync('hzw.key'),
    cert: fs.readFileSync('hzw.cert')
}

https.createServer(options, app.callback()).listen(3001)
