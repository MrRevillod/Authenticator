
import * as userServices from "../services/user.js"

import { toast } from "sonner"
import { useAuth } from "./authContext.jsx"
import { useState } from "react"
import { createContext, useContext } from "react"

const userContext = createContext()

export const useUser = () => {
    const context = useContext(userContext)

    if (!context) throw new Error("useUser debe estar dentro del proveedor UserContext")
    return context
}
export const UserProvider = ({ children }) => {

    const [isLoading, setIsLoading] = useState(false)
    const { user, setUser } = useAuth()

    const useUpdate = async (id, values) => {

        try {

            setIsLoading(true)

            const res = await userServices.updateProfile(id, values)

            setUser(res.data.user)

            toast.success(res.data.message, {
                duration: 3000,
                style: { fontSize: "1rem" }
            })

        } catch (error) {

            toast.error(error.response.data.message, {
                duration: 3000,
                style: { fontSize: "1rem" }
            })
        }

        finally {
            console.log(user)
            setIsLoading(false)
        }
    }

    return (

        <userContext.Provider

            value={{
                isLoading,
                useUpdate,
            }}
        >
            {children}

        </userContext.Provider>
    )
}


