
import * as auth from "../services/auth.js"

import { toast } from "sonner"
import { useUserStore } from "../lib/store.js"
import { useEffect, useState } from "react"
import { createContext, useContext } from "react"

const AuthContext = createContext()

export const useAuth = () => {

    const context = useContext(AuthContext)

    if (!context) throw new Error("useAuth debe estar dentro del proveedor AuthContext")
    return context
}

export const AuthProvider = ({ children }) => {

    const [isLoading, setIsLoading] = useState(false)
    const [isAuthenticated, setIsAuthenticated] = useState(false)
    const [isCheckingSession, setIsCheckingSession] = useState(true)

    const useLogin = async (formData) => {

        try {

            setIsLoading(true)

            const res = await auth.loginRequest(formData)

            setIsAuthenticated(res.status === 200)
            useUserStore.setState({ user: res.data.user })

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

            return res

        } catch (error) {

            if (error.response.status !== 409) {

                toast.error(error.response.data.message, {
                    duration: 3000,
                    style: { fontSize: "1rem" }
                })
            }

            setIsLoading(false)
            return error.response
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
            useUserStore.setState({ user: null })
        }
    }

    const checkSession = async () => {

        try {

            setIsLoading(true)
            setIsCheckingSession(true)

            const res = await auth.ValidateSessionRequest()

            setIsAuthenticated(res.status === 200)
            useUserStore.setState({ user: res.data.user })

        } catch (error) {

            setIsAuthenticated(false)
            useUserStore.setState({ user: null })

        } finally {
            setIsLoading(false)
            setIsCheckingSession(false)
        }
    }

    const useValidateAccount = async (id, token) => {

        let values = {}

        try {

            setIsLoading(true)

            const res = await auth.validateAccountRequest(id, token)

            values.isValidated = true
            values.message = res.data.message

        } catch (error) {

            values.isValidated = false
            values.message = error.response.data.message

        } finally {

            setIsLoading(false)
            return values
        }
    }

    useEffect(() => { checkSession() }, [])

    return (

        <AuthContext.Provider value={{
            isAuthenticated,
            isLoading,
            isCheckingSession,
            useValidateAccount,
            useLogin,
            useRegister,
            useLogout,
            checkSession,
        }}>
            {children}
        </AuthContext.Provider>
    )
}
