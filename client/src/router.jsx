
import { Loading } from "./pages/Loading.jsx"
import { useAuth } from "./context/useAuth.jsx"
import { Navigate, Outlet } from "react-router-dom"

export const ProtectedRoute = () => {

    const { isAuthenticated, isLoading, isCheckingSession } = useAuth()

    if (isCheckingSession) {
        return <Loading />
    }

    if (isLoading || isAuthenticated === false) {
        return <Navigate to="/auth/login" replace />
    }

    return <Outlet />
}
