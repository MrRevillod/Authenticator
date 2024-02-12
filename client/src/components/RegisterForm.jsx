
import { useForm } from "react-hook-form"
import { useEffect } from "react"
import { zodResolver } from "@hookform/resolvers/zod"
import { Link, useNavigate } from "react-router-dom"

import { Input } from "./ui/Input.jsx"
import { Spinner } from "./ui/Spinner.jsx"
import { useAuth } from "../context/authContext.jsx"
import { registerSchema } from "../lib/schemas.js"

export const RegisterForm = () => {

    const { register, handleSubmit, formState: { errors }, reset } = useForm({
        resolver: zodResolver(registerSchema)
    })

    const navigate = useNavigate()

    const { useRegister, isAuthenticated, isLoading } = useAuth()

    const onSubmit = async (formData) => {
        await useRegister(formData)
        reset()
    }

    useEffect(() => { if (isAuthenticated) navigate('/dashboard') }, [isAuthenticated])

    return (

        <div className="flex flex-col justify-center px-12 md:px-0 lg:px-0 h-3/5 w-full md:w-7/12 lg:w-3/5 items-center">

            {isLoading && (<Spinner classes={"z-10 fixed opacity-100"} />)}

            <div className={`flex flex-col justify-center mdpx-12 gap-8 w-full h-full ${isLoading ? "opacity-50" : ""}`}>

                <div className="flex flex-col items-center gap-2">
                    <h2 className="text-3xl font-bold text-neutral-100 text-center">
                        Crear una cuenta
                    </h2>

                    <p className="text-center font-light text-neutral-300 text-sm">
                        Registrate y comienza a trabajar en tus proyectos individuales o grupales.
                    </p>
                </div>

                <form className="flex flex-col gap-4 h-11/12" onSubmit={handleSubmit(onSubmit)}>

                    <Input
                        label="Nombre completo"
                        type="text"
                        placeholder="John Doe"
                        {...register('name')}
                        error={errors.name ? (errors.name.message) : ""}
                    />

                    <div className="flex md:flex-row flex-col gap-2 w-full">
                        <Input
                            label="Correo electrónico"
                            type="email"
                            placeholder="john@domain.com"
                            {...register('email')}
                            error={errors.email ? (errors.email.message) : ""}
                        />

                        <Input
                            label="Apodo"
                            type="text"
                            placeholder="J. Doe"
                            {...register('username')}
                            error={errors.username ? (errors.username.message) : ""}
                        />
                    </div>

                    <Input
                        label="Contraseña"
                        type="password"
                        placeholder="●●●●●●●●●●"
                        {...register('password')}
                        error={errors.password ? (errors.password.message) : ""}
                    />

                    <Input
                        label="Confirmar Contraseña"
                        type="password"
                        placeholder="●●●●●●●●●●"
                        {...register('confirmPassword')}
                        error={errors.confirmPassword ? (errors.confirmPassword.message) : ""}
                    />

                    <button
                        type="submit"
                        className="bg-neutral-100 text-neutral-950 rounded-lg p-2 font-bold mt-4"
                    >
                        Crear cuenta
                    </button>

                    <div className="flex justify-center">
                        <Link to="/auth/login" className="text-neutral-100 font-semibold">
                            ¿Ya tienes una cuenta? Inicia sesión
                        </Link>
                    </div>
                </form>

            </div>
        </div>
    )
}
