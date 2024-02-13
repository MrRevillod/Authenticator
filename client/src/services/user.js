
import { api } from "../lib/axios.js"

export const updateProfile = async (id, formData) => api.patch(`/users/${id}`, formData)
export const updateEmail = async (id, token) => api.get(`/users/update-email/${id}/${token}`)
export const deleteAccount = async (id) => api.delete(`/users/${id}`)