
import { api } from "../lib/axios.js"

export const loginRequest = async (formData) => api.post("/auth/login", formData)
export const registerRequest = async (formData) => api.post("/auth/register", formData)
export const logoutRequest = async () => api.post("/auth/logout")

export const validateSessionRequest = async () => api.post("/auth/validate")
export const validateAccountRequest = async (id, token) => api.post(`/account/validate/${id}/${token}`)

export const resetPasswordRequest = async (formData) => api.post("/auth/reset-password", formData)
export const validateResetPassword = async (id, token) => api.post(`/auth/reset-password/${id}/${token}`)
export const resetPassword = async (id, token, formData) => api.patch(`/auth/reset-password/${id}/${token}`, formData)