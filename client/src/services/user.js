
import { api } from "../lib/axios.js"

export const deleteAccount = async (id) => api.delete(`/account/${id}`)
export const updateAccount = async (id, formData) => api.patch(`/account/${id}`, formData)
export const updateEmail = async (id, token) => api.get(`/account/update-email/${id}/${token}`)

export const updateProfilePicture = async (id, formData, fileSize) => {

    const config = {
        headers: {
            "Content-Type": "multipart/form-data",
            "Content-Length": fileSize
        }
    }

    return api.patch(`/account/profile-picture/${id}`, formData, config)
}