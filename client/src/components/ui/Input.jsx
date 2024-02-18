
import React, { useState } from "react"

import { EyeIcon } from "./Icons.jsx"
import { Link } from "react-router-dom"

export const Input = React.forwardRef((props, ref) => {

    const { label, type, placeholder, error, name, islogin = false } = props

    const classes = `bg-neutral-950 border-1 border-neutral-500 rounded-lg p-2
        focus:outline-none focus:ring-2 focus:ring-neutral-500
        h-12 w-full pl-4 placeholder-neutral-400 text-neutral-100
    `
    const [inputType, setInputType] = useState(type)

    const togglePasswordVisibility = () => {
        setInputType(inputType === "password" ? "text" : "password")
    }

    return (

        <div className="flex flex-col gap-3 w-full">

            {(label === "Contraseña" && type === "password" && islogin) ? (

                <div className="flex justify-between w-full items-center mt-2">

                    <label htmlFor={name} className="font-semibold text-neutral-100">
                        {label}
                    </label>
                    <Link to="/auth/reset-password" className="text-neutral-100 text-sm hover:underline hover:text-blue-500">
                        ¿Olvidaste tu contraseña?
                    </Link>

                </div>

            ) : (

                <label htmlFor={name} className="font-semibold text-neutral-100">
                    {label}
                </label>
            )}

            <div className="relative flex flex-row justify-center">
                <input
                    ref={ref}
                    className={classes}
                    placeholder={placeholder}
                    {...props}
                    type={inputType}
                />

                {type === "password" && (
                    <EyeIcon
                        open={inputType === "text"}
                        onClick={togglePasswordVisibility}
                    />
                )}

            </div>

            {error && <div className="text-red-500 text-sm">{error}</div>}
        </div>
    )
})

Input.displayName = 'Input'

