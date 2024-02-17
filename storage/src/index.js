
import cors from "cors"
import express from "express"
import router from "./router.js"
import fileUpload from "express-fileupload"

import { STORAGE_PORT } from "./config/config.js"
import { connectDatabase } from "./config/database.js"

const app = express()

app.use(cors({ origin: "*" }))

app.use(fileUpload())
app.use(express.json())
app.use(express.urlencoded({ extended: true }))

app.use(express.static("public"))
app.use(router)

connectDatabase().then(() => {

    console.log("📦 Storage service connected to Database")

    app.listen(STORAGE_PORT, () => {
        console.log(`💾 Storage service running on port ${STORAGE_PORT}`)
    })

}).catch(() => {
    console.log("📦 Storage service failed on connect to Database")
})
