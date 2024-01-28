
import React from "react"
import ReactDOM from "react-dom/client"

import './index.css'

import { Home } from "./pages/Home.jsx"
import { Dashboard } from "./pages/Dashboard.jsx"
import { LoginPage } from "./pages/auth/Login.jsx"
import { RegisterPage } from "./pages/auth/Register.jsx"

import { Toast } from "./components/ui/Toast.jsx"
import { AuthProvider } from "./context/useAuth.jsx"
import { ProtectedRoute } from "./router.jsx"
import { AccountValidation } from "./pages/auth/AccountValidation.jsx"
import { BrowserRouter, Routes, Route } from "react-router-dom"

const root = ReactDOM.createRoot(document.getElementById("root"))

root.render(

    <React.StrictMode>

        <AuthProvider>

            <BrowserRouter>
                <Routes>

                    <Route path="/" element={<Home />} />
                    <Route path="/auth/login" element={<LoginPage />} />
                    <Route path="/auth/register" element={<RegisterPage />} />
                    <Route path="/auth/validate/:id/:token" element={<AccountValidation />} />

                    <Route element={<ProtectedRoute />}>
                        <Route path="/dashboard" element={<Dashboard />} />
                        <Route path="/protected" element={<h1>Protected</h1>} />
                    </Route>

                </Routes>
            </BrowserRouter>

        </AuthProvider>

        <Toast />

    </React.StrictMode>
)
