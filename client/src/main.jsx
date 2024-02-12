
import React from "react"
import ReactDOM from "react-dom/client"

import './index.css'

import { Home } from "./pages/Home.jsx"
import { Dashboard } from "./pages/Dashboard.jsx"
import { LoginPage } from "./pages/auth/Login.jsx"
import { MainLayout } from "./layouts/MainLayout.jsx"
import { RegisterPage } from "./pages/auth/Register.jsx"

import { Toast } from "./components/ui/Toast.jsx"
import { Profile } from "./pages/Profile.jsx"
import { AuthProvider } from "./context/authContext.jsx"
import { ProtectedRoute } from "./router.jsx"
import { AccountValidation } from "./pages/auth/AccountValidation.jsx"
import { BrowserRouter, Routes, Route } from "react-router-dom"
import { UserProvider } from "./context/userContext.jsx"

const root = ReactDOM.createRoot(document.getElementById("root"))

root.render(

    <React.StrictMode>

        <AuthProvider>
            <UserProvider>

                <BrowserRouter>
                    <Routes>

                        <Route path="/" element={<MainLayout><Home /></MainLayout>} />
                        <Route path="/auth/login" element={<LoginPage />} />
                        <Route path="/auth/register" element={<RegisterPage />} />
                        <Route path="/auth/validate/:id/:token" element={<AccountValidation />} />

                        <Route element={<ProtectedRoute />}>
                            
                            <Route path="/dashboard" element={
                                <MainLayout>
                                    <Dashboard />
                                </MainLayout>
                            } />

                            <Route path="/profile" element={
                                <MainLayout>
                                    <Profile />
                                </MainLayout>
                            } />

                        </Route>

                    </Routes>
                </BrowserRouter>

            </UserProvider>
        </AuthProvider>

        <Toast />

    </React.StrictMode>
)
