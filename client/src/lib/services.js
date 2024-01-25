
import { api } from "./axios"

export const loginRequest = async (formData) => api.post("/auth/login", formData)
export const registerRequest = async (formData) => api.post("/auth/register", formData)

export const logoutRequest = async () => api.post("/auth/logout")
export const refreshRequest = async () => api.get("/auth/refresh")
export const validateSessionRequest = async () => api.post("/auth/validate-session")
