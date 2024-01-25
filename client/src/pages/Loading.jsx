
import { Spinner } from "../components/ui/Spinner.jsx"

export const Loading = () => {

    return (

        <div className="h-screen w-screen flex items-center justify-center gap-4 bg-neutral-950">
            <h1 className="text-5xl text-neutral-100 font-bold">Cargando...</h1>
            <Spinner />
        </div>
    )
}