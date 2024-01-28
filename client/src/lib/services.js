
import { api } from "./axios"

export const loginRequest = async (formData) => api.post("/auth/login", formData)
export const registerRequest = async (formData) => api.post("/auth/register", formData)
export const logoutRequest = async () => api.post("/auth/logout")

export const ValidateSessionRequest = async () => api.post("/auth/validate-session")
export const validateAccountRequest = async (id, token) => api.post(`/auth/validate-account/${id}/${token}`)

export const getProtectedData = async () => api.post("/auth/protected")
