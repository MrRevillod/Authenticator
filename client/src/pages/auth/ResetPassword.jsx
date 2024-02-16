
import { z } from "zod"
import { Input } from "../../components/ui/Input.jsx"
import { useForm } from "react-hook-form"
import { useAuth } from "../../context/authContext.jsx"
import { Spinner } from "../../components/ui/Spinner.jsx"
import { zodResolver } from "@hookform/resolvers/zod"
import { useParams, Link } from "react-router-dom"
import { useState, useEffect } from "react"

const emailSchema = z.object({
    email: z.string()
        .min(1, { message: "El correo electrónico es requerido" })
        .email({ message: "El correo electrónico no es válido" }),
})

const passwordSchema = z.object({
    password: z.string()
        .min(8, { message: "La contraseña debe tener al menos 8 caracteres" })
        .max(30, { message: "La contraseña debe tener menos de 30 caracteres" })
        .regex(/^(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*\W).{8,30}$/, { message: "La contraseña debe tener al menos un número, una letra mayúscula, una minúscula y un carácter especial" }),

    confirmPassword: z.string()
        .min(8, { message: "La contraseña debe tener al menos 8 caracteres" })
        .max(30, { message: "La contraseña debe tener menos de 30 caracteres" })
        .regex(/^(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*\W).{8,30}$/, { message: "La contraseña debe tener al menos un número, una letra mayúscula, una minúscula y un carácter especial" })
})
    .refine((data) => data.password === data.confirmPassword, {
        message: "Las contraseñas no coinciden",
        path: ["confirmPassword"],
    })

export const ForgotPasswordRequestPage = () => {

    const { register, handleSubmit, formState: { errors } } = useForm({
        resolver: zodResolver(emailSchema)
    })

    const { isLoading, useRequestResetPassword } = useAuth()

    const onSubmit = async (formData) => {
        await useRequestResetPassword(formData)
    }

    return (

        <div className="w-full h-full flex flex-col justify-start items-center">

            {isLoading && (<Spinner classes={"z-10 fixed opacity-100"} />)}

            <div className="w-5/6 md:w-2/3 lg:w-1/3 h-full flex flex-col justify-center items-center -mt-8">
                <h1 className="text-3xl font-bold text-neutral-100 text-center mt-4">
                    Restablecer contraseña
                </h1>
                <form className="flex flex-col gap-4 p-8 w-full" onSubmit={handleSubmit(onSubmit)}>
                    <Input
                        label="Correo electrónico"
                        type="email"
                        {...register('email')}
                        placeholder="john@domain.com"
                        error={errors.email ? (errors.email.message) : ""}
                    />

                    <button
                        type="submit"
                        className="bg-neutral-100 text-neutral-950 rounded-lg p-2 font-bold mt-4"
                    >
                        Enviar
                    </button>
                </form>
            </div>
        </div>
    )
}

export const ForgotPasswordPage = () => {

    const { register, handleSubmit, formState: { errors }, reset } = useForm({
        resolver: zodResolver(passwordSchema)
    })

    const [validationRes, setValRes] = useState({})
    const { id, token } = useParams()
    const { useResetPassword, useValidateResetPassword } = useAuth()

    useEffect(() => {

        const validatePage = async () => {

            const res = await useValidateResetPassword(id, token)

            if (res.status === 401) {
                res.data.message = "URL inválida o expirada"
            }

            setValRes({ status: res.status, message: res.data.message })
        }

        validatePage()

    }, [])

    const onSubmit = async (formData) => {
        await useResetPassword(id, token, formData)
        reset()
    }

    return (

        validationRes.status === 200 ? (

            <div className="w-full h-full flex flex-col justify-start items-center" >
                <div className="w-5/6 md:w-2/3 lg:w-1/3 h-full flex flex-col justify-center items-center -mt-8">
                    <h1 className="text-3xl font-bold text-neutral-100 text-center mt-4">
                        Restablecer contraseña
                    </h1>
                    <form className="flex flex-col gap-4 p-8 w-full" onSubmit={handleSubmit(onSubmit)}>
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
                            Enviar
                        </button>
                    </form>
                </div>
            </div >

        ) : (

            <div className="w-full h-full bg-neutral-950 flex justify-center items-center flex-col gap-8">

                <h1 className="text-neutral-100 text-5xl text-center font-bold">{validationRes.message}</h1>

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

            </div>
        )
    )
}