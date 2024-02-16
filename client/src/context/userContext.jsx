
import * as userServices from "../services/user.js"

import { toast } from "sonner"
import { useState } from "react"
import { useUserStore } from "../lib/store.js"
import { createContext, useContext } from "react"

const userContext = createContext()

export const useUser = () => {
    const context = useContext(userContext)

    if (!context) throw new Error("useUser debe estar dentro del proveedor UserContext")
    return context
}

export const UserProvider = ({ children }) => {

    const [isLoading, setIsLoading] = useState(false)

    const useDeleteAccount = async (id) => {

        try {

            setIsLoading(true)

            const res = await userServices.deleteAccount(id)

            toast.success(res.data.message, {
                duration: 3000,
                style: { fontSize: "1rem" }
            })

            useUserStore.setState({ user: null })

        } catch (error) {

            toast.error(error.response.data.message, {
                duration: 3000,
                style: { fontSize: "1rem" }
            })

            return error.response
        }

        finally { setIsLoading(false) }
    }

    const useUpdate = async (id, values) => {

        try {

            setIsLoading(true)

            const res = await userServices.updateAccount(id, values)

            useUserStore.setState({ user: res.data.user })

            toast.success(res.data.message, {
                duration: 3000,
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

            return error.response
        }

        finally { setIsLoading(false) }
    }

    const useUpdateEmail = async (id, token) => {

        let values = {}

        try {

            setIsLoading(true)

            const res = await userServices.updateEmail(id, token)

            values.isChanged = true
            values.message = res.data.message

        } catch (error) {

            values.isChanged = false
            values.message = error.response.data.message
        }

        finally {

            setIsLoading(false)
            return values
        }
    }

    return (

        <userContext.Provider

            value={{
                isLoading,
                useUpdate,
                useUpdateEmail,
                useDeleteAccount,
            }}
        >
            {children}

        </userContext.Provider>
    )
}


