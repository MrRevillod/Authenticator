
import { RegisterForm } from "../../components/RegisterForm.jsx"

export const RegisterPage = () => {

    return (

        <main className="h-screen w-screen bg-neutral-950 flex items-center justify-center gap-8">

            <article className="w-1/2 h-full lg:flex items-center justify-center hidden auth-bg-img">

                <h1 className="text-5xl font-bold text-neutral-100 text-center">
                    Bienvenido a Authenticator
                </h1>

            </article>

            <article className="lg:w-1/2 w-full h-full lg:px-0 px-4 md:px-0 flex items-center justify-center">
                <RegisterForm />
            </article>

        </main>
    )
}