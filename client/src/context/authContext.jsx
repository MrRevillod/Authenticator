
import * as auth from "../services/auth.js"

import { toast } from "sonner"
import { useEffect, useState } from "react"
import { createContext, useContext } from "react"

const AuthContext = createContext()

export const useAuth = () => {

    const context = useContext(AuthContext)

    if (!context) throw new Error("useAuth debe estar dentro del proveedor AuthContext")
    return context
}

export const AuthProvider = ({ children }) => {

    const [user, setUser] = useState(null)
    const [isLoading, setIsLoading] = useState(false)
    const [isAuthenticated, setIsAuthenticated] = useState(false)
    const [isCheckingSession, setIsCheckingSession] = useState(true)

    const useLogin = async (formData) => {

        try {

            setIsLoading(true)

            const res = await auth.loginRequest(formData)

            setIsAuthenticated(res.status === 200)
            setUser(res.data.user)

        } catch (error) {

            setIsAuthenticated(false)

            toast.error(error.response.data.message, {
                duration: 3000,
                style: { fontSize: "1rem" }
            })

        } finally {
            setIsLoading(false)
        }
    }

    const useRegister = async (formData) => {

        try {

            setIsLoading(true)

            const res = await auth.registerRequest(formData)

            setIsLoading(false)

            toast.success(res.data.message, {
                duration: 5000,
                style: { fontSize: "1rem" }
            })

        } catch (error) {

            toast.error(error.response.data.message, {
                duration: 3000,
                style: { fontSize: "1rem" }
            })

        } finally {
            setIsLoading(false)
        }
    }

    const useLogout = async () => {

        try {

            const res = await auth.logoutRequest()

            setIsAuthenticated(!(res.status === 200))

            toast.success(res.data.message, {
                duration: 5000,
                style: { fontSize: "1rem" }
            })

        } catch (error) {

            setIsAuthenticated(false)

            toast.error("Sesión cerrada", {
                duration: 3000,
                style: { fontSize: "1rem" }
            })

        } finally {
            setIsLoading(false)
        }
    }

    const checkSession = async () => {

        try {

            setIsLoading(true)
            setIsCheckingSession(true)

            const res = await auth.ValidateSessionRequest()

            setIsAuthenticated(res.status === 200)
            setUser(res.data.user)

        } catch (error) {

            setIsAuthenticated(false)
            setUser(null)

        } finally {
            setIsLoading(false)
            setIsCheckingSession(false)
        }
    }

    useEffect(() => { checkSession() }, [])
    
    return (

        <AuthContext.Provider value={{
            isAuthenticated,
            isLoading,
            isCheckingSession,
            user,
            setUser,
            useLogin,
            useRegister,
            useLogout,
            checkSession,
        }}>
            {children}
        </AuthContext.Provider>
    )
}
