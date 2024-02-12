
import { useAuth } from "../context/authContext.jsx"
import { useNavigate } from "react-router-dom"

export const LogoutButton = () => {

    const navigate = useNavigate()
    const { useLogout } = useAuth()

    const handleLogout = async () => {
        await useLogout()
        navigate('/auth/login')
    }

    return (
        <button 
            type="submit"
            className="text-neutral-100 border-1 rounded-lg p-2 w-52 h-12 font-bold"
            onClick={handleLogout}
        >
            Cerrar sesión 
        </button>
    )
}