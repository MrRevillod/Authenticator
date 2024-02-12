
import { useAuth } from "../../context/authContext.jsx"
import { Link, useNavigate } from "react-router-dom"
import { useEffect, useState } from "react"
import { validateAccountRequest } from "../../services/auth.js"

export const AccountValidation = () => {

    const navigate = useNavigate()

    const { isAuthenticated } = useAuth()
    const [message, setMessage] = useState("")
    const [isValidated, setIsValidated] = useState(false)

    useEffect(() => { if (isAuthenticated) navigate("/dashboard") }, [isAuthenticated])

    useEffect(() => {

        const validateAccount = async () => {
            
            const path = window.location.pathname.split("/")
    
            const id = path[3]
            const token = path[4]

            try {
                const res = await validateAccountRequest(id, token)
                setMessage(res.data.message)
                setIsValidated(true)
            
            } catch (error) {
                setMessage(error.response.data.message)
                setIsValidated(false)
            }
            
        }

        validateAccount()
    
    }, [])

    return (

        <main className="w-screen h-screen bg-neutral-950 flex justify-center items-center flex-col gap-8">

            <h1 className="text-neutral-100 text-5xl text-center font-bold">{message}</h1>

            {isValidated && 
                
                <Link 
                    to="/auth/login" 
                    className="text-neutral-100 text-xl text-center font-semibold border-1 border-neutral-100 rounded-md px-4 py-2 mt-4 hover:bg-neutral-100 hover:text-neutral-950 transition duration-300 ease-in-out"
                >
                    Iniciar sesión

                </Link>
            }
            
        </main>
    )
}