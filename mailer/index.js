
import cors from "cors"
import express from "express"

import { router } from "./router.js"
import { API_URL } from "./config.js"

const app = express()

app.use(cors({ origin: API_URL, methods: ["POST"] }))

app.use(express.json())
app.use(express.urlencoded({ extended: true }))

app.use(router)

app.listen(7000, () => {
    console.log("Mailer service running on port 7000")
})
