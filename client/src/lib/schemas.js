
import { z } from 'zod'

export const registerSchema = z.object({

    name: z.string()
        .min(5, { message: "El nombre debe tener al menos 5 caracteres" })
        .max(30, { message: "El nombre debe tener menos de 30 caracteres" }),

    username: z.string()
        .min(5, { message: "El apodo debe tener al menos 5 caracteres" })
        .max(20, { message: "El apodo debe tener menos de 30 caracteres" }),

    email: z.string()
        .email({ message: "El email no es válido" })
        .max(50, { message: "El email debe tener menos de 100 caracteres" }),

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