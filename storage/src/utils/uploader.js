
import path from "node:path"

import { v4 as uuid } from "uuid"
import { INPUT_DIR } from "../config/config.js"

export const fileUploader = (file) => {

    const ext = path.extname(file.name)

    file.name = `${uuid()}-${Date.now()}${ext}`

    const filePath = `${INPUT_DIR}/${file.name}`

    return new Promise((resolve, reject) => {

        file.mv(filePath, (err) => {

            if (err) {
                console.log(err)
                reject({ status: 500, message: "Error uploading" })

            } else {
                resolve(file.name)
            }
        })
    })
}
