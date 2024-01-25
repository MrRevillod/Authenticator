
import { useAuth } from "./context/useAuth"
import { Navigate, Outlet } from "react-router-dom"

import { Loading } from "./pages/Loading.jsx"

export const ProtectedRoute = () => {
    
    const { isAuthenticated, isLoading } = useAuth()
    
    if (isLoading) return <Loading />
    
    if (!isAuthenticated && !isLoading) {
        return <Navigate to="/auth/login" replace />
    }
   
    return <Outlet />
}
