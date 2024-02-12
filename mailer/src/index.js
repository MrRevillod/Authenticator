
import cors from "cors"
import express from "express"

import { router } from "./router.js"
import { MAILER_PORT } from "./config.js"

const app = express()

//import { API_URL } from "./config.js"
//app.use(cors({ origin: API_URL, methods: ["POST"] }))

app.use(cors({ origin: "*" }))

app.use(express.json())
app.use(express.urlencoded({ extended: true }))

app.use(router)

app.listen(MAILER_PORT, () => {
    console.log(`Mailer service running on port: ${MAILER_PORT}`)
})
