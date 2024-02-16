
import { useAuth } from "../context/authContext.jsx"
import { Link, useLocation } from "react-router-dom"

import "../index.css"

export const Navbar = () => {

    const location = useLocation()
    const isDashboardOrHome = location.pathname === "/" || location.pathname === "/dashboard"

    const { isAuthenticated, useLogout } = useAuth()

    return (

        <div className="navbar bg-neutral-950 py-8 px-6 md:px-12 lg:px-28 fixed top-0 w-full z-50 h-28">
            <div className="flex-1">

                {isDashboardOrHome && (

                    <Link
                        to="/"
                        className="
                        text-neutral-300 font-bold text-2xl hover:text-neutral-400 transition duration-200
                    "
                    >
                        Workflow

                    </Link>
                )}

                {!isDashboardOrHome && (

                    <Link
                        to="/dashboard"
                        className="
                            text-neutral-300 flex items-center justify-center font-semibold 
                            text-xl hover:text-neutral-400 transition duration-200 back
                        "
                    >

                        volver

                    </Link>
                )}

            </div>
            <div className="flex-none">

                {isAuthenticated && (

                    <div className="dropdown dropdown-end">
                        <div tabIndex={0} role="button" className="btn btn-ghost btn-circle avatar">
                            <div className="w-10 rounded-full">
                                <img alt="Tailwind CSS Navbar component" src="/default-profile.png" />
                            </div>
                        </div>
                        <ul tabIndex={0} className="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-neutral-900 rounded-lg w-52">
                            {location !== "/dashboard" && (<li><Link to="/dashboard">Página principal</Link></li>)}
                            <li><Link to="/account/profile">Mi perfíl</Link></li>
                            <li><Link onClick={useLogout}>Cerrar sesión</Link></li>
                        </ul>
                    </div>
                )}

                {!isAuthenticated && (

                    <Link
                        to="/auth/login"
                        className="bg-neutral-100 rounded-lg px-8 py-2 text-neutral-950 font-bold text-md"
                    >
                        Iniciar sesión

                    </Link>
                )}

            </div>
        </div>
    )
}

