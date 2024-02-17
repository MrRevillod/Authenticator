
import sharp from "sharp"

export const optimize = async (filePath, outputPath) => {

    return new Promise((resolve, reject) => {

        sharp(filePath).toFormat("webp").webp({ quality: 60 })

            .toFile(outputPath, (err, _info) => {

                if (err) reject({ status: 500, message: "Internal server error" })

                resolve()
            })
    })
}