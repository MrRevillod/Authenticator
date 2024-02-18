
import { MESSAGES } from "./utils/responses.js"
import { MIME_TYPES } from "./config/config.js"

export const fileValidator = (req, res, next) => {

    try {

        if (!req.files || Object.keys(req.files).length === 0 || !req.files.file) {
            throw { status: 400, message: MESSAGES.UPLOAD_NOT_FOUND }
        }

        const file = req.files.file

        if (!MIME_TYPES.includes(file.mimetype)) {
            console.log(file.mimetype)
            throw { status: 400, message: MESSAGES.INVALID_FILE_TYPE }
        }

        next()
    }

    catch (error) {
        return res.status(error?.status || 500).json({ message: error?.message || MESSAGES.INTERNAL_ERROR })
    }
}