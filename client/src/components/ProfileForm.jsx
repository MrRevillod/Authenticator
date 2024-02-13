
import { useForm } from "react-hook-form"
import { useState } from "react"
import { zodResolver } from "@hookform/resolvers/zod"

import { toast } from "sonner"
import { Input } from "./ui/Input.jsx"
import { Spinner } from "./ui/Spinner.jsx"
import { useUser } from "../context/userContext.jsx"
import { ConfirmModal } from "./ConfirmModal.jsx"
import { useUserStore } from "../lib/store.js"
import { profileSchema } from "../lib/schemas.js"

export const ProfileForm = () => {

    const user = useUserStore(state => state.user)
    const { register, handleSubmit, formState: { errors }, setError, getValues, reset } = useForm({
        resolver: zodResolver(profileSchema)
    })

    const { useUpdate, useDeleteAccount, isLoading } = useUser()
    const [isModalOpen, setIsModalOpen] = useState(false)

    const handleDelete = async () => {

        setIsModalOpen(false)
        const id = user._id
        await useDeleteAccount(id)
        location.reload()
    }

    const onSubmit = async (formData) => {

        const formValues = getValues()

        const values = {}

        for (const key in formValues) {

            if (formValues[key] !== user[key] && formValues[key] !== "") {
                values[key] = formValues[key]
            }
        }

        if (Object.keys(values).length === 0) {
            toast.error("No se han hecho cambios", {
                duration: 3000,
                style: { fontSize: "1rem" }
            })
            return
        }

        let response = await useUpdate(user._id, values)

        if (response.status === 409) {

            if (response.data.conflicts.username) {

                setError("username", {
                    type: "manual",
                    message: "El apodo ya está en uso"
                })
            }

            if (response.data.conflicts.email) {

                setError("email", {
                    type: "manual",
                    message: "El correo electrónico ya está en uso"
                })
            }

            return
        }

        reset()
    }

    return (

        <div className="flex flex-col justify-center px-12 md:px-0 lg:px-0 h-3/5 w-full lg:w-3/5 items-center">

            {isLoading && (<Spinner classes={"z-10 fixed opacity-100"} />)}

            <div className={`flex flex-col justify-center mdpx-12 gap-5 w-full h-full ${isLoading ? "opacity-50" : ""}`}>

                <form className="flex flex-col gap-5 h-11/12" onSubmit={handleSubmit(onSubmit)}>

                    <Input
                        label="Nombre"
                        type="text"
                        placeholder={user.name}
                        {...register('name')}
                        error={errors.name ? (errors.name.message) : ""}
                    />

                    <Input
                        label="Apodo"
                        type="text"
                        placeholder={user.username}
                        {...register('username')}
                        error={errors.username ? (errors.username.message) : ""}
                    />

                    <Input
                        label="Correo eléctronico"
                        type="email"
                        placeholder={user.email}
                        {...register('email')}
                        error={errors.email ? (errors.email.message) : ""}
                    />

                    <div className="flex flex-row gap-4">

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
                    </div>


                    <button
                        type="submit"
                        className="bg-neutral-100 text-neutral-950 rounded-lg p-2 font-bold mt-4"
                    >
                        Actualizar perfíl
                    </button>


                </form>

                <button
                    onClick={() => setIsModalOpen(true)}
                    className="bg-red-700 text-neutral-200 rounded-lg p-2 font-bold"
                >
                    Eliminar cuenta
                </button>

                <ConfirmModal
                    isOpen={isModalOpen}
                    text="¿Estás seguro de que quieres eliminar tu cuenta?"
                    onConfirm={handleDelete}
                    onClose={() => setIsModalOpen(false)}
                />

            </div>
        </div>
    )
}
