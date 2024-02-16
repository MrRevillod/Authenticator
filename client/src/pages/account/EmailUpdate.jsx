
import { Link } from "react-router-dom"
import { Spinner } from "../../components/ui/Spinner.jsx"
import { useUser } from "../../context/userContext.jsx"
import { useEffect, useState } from "react"

export const EmailUpdatePage = () => {

    const [res, setRes] = useState({})
    const { useUpdateEmail, isLoading } = useUser()

    useEffect(() => {

        const emailUpdate = async () => {

            const path = window.location.pathname.split("/")

            const id = path[3]
            const token = path[4]
            const res = await useUpdateEmail(id, token)

            setRes(res)
        }

        emailUpdate()

    }, [])

    return (

        <main className="w-screen h-screen bg-neutral-950 flex justify-center items-center flex-col gap-8">

            {isLoading && (<Spinner classes={"z-10 fixed opacity-100"} />)}

            <h1 className="text-neutral-100 text-5xl text-center font-bold">{res.message}</h1>

            <Link
                to="/dashboard"
                className="
                    text-neutral-100 text-xl text-center font-semibold border-1 
                    border-neutral-100 rounded-md px-4 py-2 mt-4 hover:bg-neutral-100 
                    hover:text-neutral-950 transition duration-300 ease-in-out
                "
            >
                Volver al dashboard
            </Link>

        </main>
    )
}

