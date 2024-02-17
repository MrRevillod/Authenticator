
import { MESSAGES } from "./utils/responses.js"
import { MIME_TYPES } from "./config/config.js"
import { userModel } from "./utils/schemas.js"

import path from "node:path"

export const fileValidator = (req, res, next) => {

    try {

        if (!req.files || Object.keys(req.files).length === 0 || !req.files.file) {
            throw { status: 400, message: MESSAGES.UPLOAD_NOT_FOUND }
        }

        const file = req.files.file
        const fileExtension = path.extname(file.name)

        if (!MIME_TYPES.includes(fileExtension)) {
            throw { status: 400, message: MESSAGES.INVALID_FILE_TYPE }
        }

        next()
    }

    catch (error) {
        return res.status(error?.status || 500).json({ message: error?.message || MESSAGES.INTERNAL_ERROR })
    }
}

export const checkUserId = async (req, res, next) => {

    const { id } = req.params

    try {

        const user = await userModel.findById(id)

        if (!user) {
            throw { status: 404, message: MESSAGES.RESOURCE_NOT_FOUND }
        }

        req.user = user

        next()

    } catch (error) {
        return res.status(error?.status || 500).json({ message: error?.message || MESSAGES.INTERNAL_ERROR })
    }
}