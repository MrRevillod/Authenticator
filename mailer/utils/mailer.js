
import { createTransport } from "nodemailer"
import { MAIL_ADRESS, MAIL_PASSWORD } from "../config.js"

export const transporter = createTransport({

    service: "gmail",
    auth: {
        user: MAIL_ADRESS,
        pass: MAIL_PASSWORD
    }
})

export const mailValidator = (mail) => {
    const regex = /^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}$/
    return regex.test(mail)
}