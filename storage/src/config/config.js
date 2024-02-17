
import path from "node:path"

const API_URL = process.env.API_URL || "http://localhost:1000"
const STORAGE_KEY = process.env.STORAGE_KEY || "storage-api-key"
const STORAGE_PORT = process.env.STORAGE_PORT || 2000
const STORAGE_URL = `http://localhost:${STORAGE_PORT}`

const DB_NAME = process.env.DB_NAME || "storage"
const DB_URI = process.env.DB_URI || "mongodb://localhost:27017"

const INPUT_DIR = path.join(process.cwd(), "input")

const MIME_TYPES = [".png", ".jpg", ".webp", ".jpeg"]

export {
    API_URL, STORAGE_KEY,
    STORAGE_PORT, MIME_TYPES,
    STORAGE_URL, INPUT_DIR,
    DB_NAME, DB_URI
}