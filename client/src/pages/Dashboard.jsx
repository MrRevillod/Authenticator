
import { useEffect } from "react"
import { LogoutButton } from "../components/LogoutButton.jsx"

import { Link } from "react-router-dom"
import { useAuth } from "../context/useAuth.jsx"

export const Dashboard = () => {

    const { user, fetchProtectedData } = useAuth()

    useEffect(() => {

        const getData = async () => {
            console.log(user)
            await fetchProtectedData()
        }

        getData()

    }, [])

    return (
        <div className="h-screen w-screen bg-neutral-950 flex items-center justify-center flex-col gap-8">
            <h1 className="text-5xl text-neutral-100 font-bold">Dashboard</h1>

            <LogoutButton />

            <Link to="/protected" className="text-neutral-100 font-bold">protected</Link>
        </div>
    )
}