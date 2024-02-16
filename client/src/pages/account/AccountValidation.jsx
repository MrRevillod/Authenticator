
import { useAuth } from "../../context/authContext.jsx"
import { useEffect, useState } from "react"
import { Link, useNavigate } from "react-router-dom"

export const AccountValidationPage = () => {

    const navigate = useNavigate()

    const [res, setRes] = useState({})
    const { isAuthenticated, useValidateAccount } = useAuth()

    useEffect(() => { if (isAuthenticated) navigate("/dashboard") }, [isAuthenticated])

    useEffect(() => {

        const validateAccount = async () => {

            const path = window.location.pathname.split("/")

            const id = path[3]
            const token = path[4]

            const res = await useValidateAccount(id, token)

            setRes(res)
        }

        validateAccount()

    }, [])

    return (

        <main className="w-screen h-screen bg-neutral-950 flex justify-center items-center flex-col gap-8">

            <h1 className="text-neutral-100 text-5xl text-center font-bold">{res.message}</h1>

            <Link

                to="/auth/login"
                className="
                    text-neutral-100 text-xl text-center font-semibold border-1 
                    border-neutral-100 rounded-md px-4 py-2 mt-4 hover:bg-neutral-100 
                    hover:text-neutral-950 transition duration-300 ease-in-out
                "
            >
                Iniciar sesión

            </Link>

        </main>
    )
}

