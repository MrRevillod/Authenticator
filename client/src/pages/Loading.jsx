
import { Spinner } from "../components/ui/Spinner.jsx"

export const Loading = () => {

    return (

        <div className="h-screen w-screen flex items-center justify-center gap-4 bg-neutral-950">
            <Spinner />
        </div>
    )
}