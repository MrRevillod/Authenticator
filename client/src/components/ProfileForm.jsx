
import { Link } from "react-router-dom"
import { useForm } from "react-hook-form"
import { zodResolver } from "@hookform/resolvers/zod"

import { Input } from "./ui/Input.jsx"
import { useAuth } from "../context/useAuth.jsx"
import { Spinner } from "./ui/Spinner.jsx"
import { profileSchema } from "../lib/schemas.js"

export const RegisterForm = () => {

    const { register, handleSubmit, formState: { errors }, reset } = useForm({
        resolver: zodResolver(registerSchema)
    })

    const { isAuthenticated, isLoading } = useAuth()

    const onSubmit = async (formData) => {
        reset()
    }

    return (

        <div className="flex flex-col justify-center px-12 md:px-0 lg:px-0 h-3/5 w-full md:w-7/12 lg:w-3/5 items-center">

            {isLoading && (<Spinner classes={"z-10 fixed opacity-100"} />)}

            <div className={`flex flex-col justify-center mdpx-12 gap-8 w-full h-full ${isLoading ? "opacity-50" : ""}`}>

                <form className="flex flex-col gap-4 h-11/12" onSubmit={handleSubmit(onSubmit)}>

                </form>

            </div>
        </div>
    )
}
