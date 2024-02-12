
import { useAuth } from "../context/authContext.jsx"
import { api } from "../lib/axios.js"

export const updateProfile = async (id, formData) => api.patch(`/users/${id}`, formData)
