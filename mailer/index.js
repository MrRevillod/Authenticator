
import express, { Router } from "express"
import cors from "cors"

import { transporter, mailValidator } from "./utils/mailer.js"
import { API_URL, MAILER_KEY, MAIL_ADRESS } from "./config.js"
import { validateAccountTemplate } from "./utils/templates.js"

const app = express()
const router = Router()

app.use(cors({
    origin: API_URL,
    methods: ["POST"],
}))

app.use(express.json())
app.use(express.urlencoded({ extended: true }))


const originValidation = (req, res, next) => {

    const apiKey = req.headers["x-api-key"]

    if (apiKey !== MAILER_KEY) {
        return res.status(401).json({ message: "Unauthorized" })
    }

    next()
}

router.post("/mailer", originValidation, async (req, res) => {
    
    const { email, url } = await req.body
    
    if (!mailValidator(email)) {
        return res.status(400).json({ message: "Invalid email" })    
    }

    transporter.sendMail({
        from: `Workflow Services ${MAIL_ADRESS}`,
        to: email,
        subject: "Bienvenido a Workflow - Verifica tu correo",
        html: validateAccountTemplate(url)
    
    }).then(() => {
        return res.status(200).json({ message: "Mail sent" })
    
    }).catch((err) => {
        console.log(err)
        return res.status(500).json({ message: "Internal server error" })
    })
})


app.use(router)

app.listen(7000, () => {
    console.log("Mailer service running on port 7000")
})