import express from 'express'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const app = express()
const port = 3000

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

app.use(express.static(path.join(__dirname, 'public')))
app.use(express.raw({ type: "application/octet-stream", limit: "50mb" }))

let latestPointCloud = []
const clients = new Set()

app.post("/data", (req, res) => {
  const buf = req.body

  if (!buf || buf.length % 4 !== 0) {
    return res.status(400).json({ ok: false, error: "invalid buffer" })
  }

  const f32 = new Float32Array(buf.buffer.slice(buf.byteOffset, buf.byteOffset + buf.byteLength))

  const points = []

  for (let i = 0; i < f32.length; i += 3) {
    points.push([f32[i], f32[i + 1], f32[i + 2]])
  }

  latestPointCloud = points

  const payload = `data: ${JSON.stringify(points)}\n\n`

  for (const client of clients) {
    client.write(payload)
  }

  res.json({ ok: true, points: points.length })
})

app.get('/events', (req, res) => {
  res.writeHead(200, {
    'Content-Type': 'text/event-stream',
    'Cache-Control': 'no-cache',
    Connection: 'keep-alive'
  })

  res.flushHeaders?.()

  clients.add(res)

  // initial state
  res.write(`data: ${JSON.stringify(latestPointCloud)}\n\n`)

  req.on('close', () => {
    clients.delete(res)
  })
})

app.listen(port, () => {
  console.log(`http://localhost:${port}`)
})