
import { LogoutButton } from "../components/LogoutButton.jsx"

export const Dashboard = () => {
    
    return (
        <div className="h-screen w-screen bg-neutral-950 flex items-center justify-center flex-col gap-8">
            <h1 className="text-5xl text-neutral-100 font-bold">Dashboard</h1>

            <LogoutButton />
        </div>
    )
}