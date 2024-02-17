
import { model, Schema } from "mongoose"

const User = new Schema({

    name: {
        type: String, required: true,
    },

    username: {
        type: String, required: true, unique: true,
    },

    email: {
        type: String, required: true, unique: true,
    },

    password: {
        type: String, required: true,
    },

    validated: { type: Boolean },

    profilePicture: { type: String },
})

export const userModel = model("User", User)