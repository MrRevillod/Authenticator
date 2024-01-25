
import * as auth from "../lib/services.js"

import { toast } from "sonner"
import { useEffect } from "react"
import { createContext, useContext, useState } from "react"

const AuthContext = createContext()

export const useAuth = () => {
    
    const context = useContext(AuthContext)
    
    if (!context) throw new Error("useAuth debe estar dentro del proveedor AuthContext")
    return context
}

export const AuthProvider = ({ children, disable }) => {

    const [isAuthenticated, setIsAuthenticated] = useState(false)
    const [responseMessage, setResponseMessage] = useState("")
    
    const [isLoading, setIsLoading] = useState(true)
    const [sessionExp, setSessionExp] = useState(0)

    const useLogin = async (formData) => {

        try {

            const res = await auth.loginRequest(formData)
            setIsAuthenticated(res.status === 200)
            setSessionExp(res.data.exp)
        
        } catch (error) {
            setIsAuthenticated(false)
            toast(error.response.data.message)
        
        } finally {
            setIsLoading(false)
        }
    }

    const useRegister = async (formData) => {

        try {
            
            const res = await auth.registerRequest(formData)
            setIsAuthenticated(res.status === 201)
            toast(res.data.message)
            
        } catch (error) {
            setIsAuthenticated(false)
            toast(error.response.data.message)
        
        } finally {
            setIsLoading(false)
        }
    }

    const useLogout = async () => {

        try {
            
            const res = await auth.logoutRequest()
            setIsAuthenticated(!(res.status === 200))
            setResponseMessage(res.data.message)
            
        } catch (error) {
            setIsAuthenticated(false)
            toast(error.response.data.message)
        
        } finally {
            setIsLoading(false)
        }
    }

    const useRefresh = async () => {

        try {
            
            const res = await auth.refreshRequest()
            setIsAuthenticated(res.status === 200)
            setResponseMessage(res.data.message)
            
        } catch (error) {
            setIsAuthenticated(false)
        
        } finally {
            setIsLoading(false)
        }
    }

    useEffect(() => {

        const checkSession = async () => {

            try {
                
                setIsLoading(true)

                const res = await auth.validateSessionRequest()
                setIsAuthenticated(res.status === 200)
            
            } catch (error) {
                setIsAuthenticated(false)
            
            } finally {
                setIsLoading(false)
            }
        }

        checkSession()

    }, [])

    return (

        <AuthContext.Provider value={{
            isAuthenticated,
            responseMessage,
            isLoading,
            sessionExp,
            useLogin,
            useRegister,
            useLogout,
            useRefresh
        }}>
            {children}
        </AuthContext.Provider>
    )
}

