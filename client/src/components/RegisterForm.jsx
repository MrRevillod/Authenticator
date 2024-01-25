
import { useEffect } from "react"
import { useForm } from "react-hook-form"
import { zodResolver } from "@hookform/resolvers/zod"
import { Link, useNavigate } from "react-router-dom"

import { Input } from "./ui/Input.jsx"
import { useAuth } from "../context/useAuth.jsx"
import { registerSchema } from "../lib/schemas.js"

export const RegisterForm = () => {
    
    const { register, handleSubmit, formState: { errors }, reset } = useForm({
        resolver: zodResolver(registerSchema)
    })

    const navigate = useNavigate()
    const { useRegister, isAuthenticated } = useAuth()

    const onSubmit = async (formData) => {
        
        await useRegister(formData)

        setTimeout(() => {

            reset()
            navigate("/auth/login")
        
        }, 2000)
    }

    useEffect(() => {  
        
        if (isAuthenticated) navigate('/dashboard')
    
    }, [isAuthenticated])

    return (
        
        <div className="flex flex-col justify-center gap-8 px-12 md:px-10 h-3/5 w-full md:w-1/2 lg:w-2/3">

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
    )
}
