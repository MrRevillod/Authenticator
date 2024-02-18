
import { STORAGE_URL } from "./config/config.js"
import { optimize } from "./utils/optimizer.js"
import { fileUploader } from "./utils/uploader.js"

import fs from "node:fs/promises"
import path from "node:path"

export const updateProfilePicture = async (req, res) => {

    try {

        const id = req.params.id
        const filename = await fileUploader(req.files.file)

        const inputPath = path.join(process.cwd(), "input")
        const publicPath = path.join(process.cwd(), "public/pictures")

        const outputName = filename.split(".")[0].concat(".webp")

        await fs.mkdir(publicPath, { recursive: true })
        await optimize(`${inputPath}/${filename}`, `${publicPath}/${outputName}`)

        await fs.rm(`${inputPath}/${filename}`)

        const url = `${STORAGE_URL}/pictures/${outputName}`

        res.status(200).json({ id, url })

    } catch (error) {
        res.status(error?.status || 500).json({ message: error?.message || "Internal server error" })
    }
}
