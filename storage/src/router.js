
import { Router } from "express"
import { fileValidator } from "./middlewares.js"
import { updateProfilePicture } from "./controllers.js"

const router = Router()

router.post("/upload", fileValidator, updateProfilePicture)

export default router