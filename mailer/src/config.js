
import "dotenv/config"

export const API_URL = process.env.API_URL || "http://localhost:1000"
export const MAILER_KEY = process.env.MAILER_KEY || "mailer-api-key"
export const MAILER_PORT = process.env.MAILER_PORT || 7000

export const MAIL_ADRESS = process.env.MAIL_ADRESS || ""
export const MAIL_PASSWORD = process.env.MAIL_PASSWORD || ""

export const FROM = `Workflow Services ${MAIL_ADRESS}`