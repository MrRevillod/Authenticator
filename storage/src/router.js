
import { Router } from "express"
import { checkUserId, fileValidator } from "./middlewares.js"
import { updateProfilePicture } from "./controllers.js"

const router = Router()

router.patch("/profile-picture/:id",
    fileValidator, checkUserId, updateProfilePicture
)

export default router