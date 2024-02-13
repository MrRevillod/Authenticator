
import React from "react"
import ReactDOM from "react-dom/client"

import { BrowserRouter, Routes, Route } from "react-router-dom"

import './index.css'

import { Home } from "./pages/Home.jsx"
import { Profile } from "./pages/Profile.jsx"
import { Dashboard } from "./pages/Dashboard.jsx"
import { LoginPage } from "./pages/auth/Login.jsx"
import { EmailUpdate } from "./pages/auth/EmailUpdate.jsx"
import { RegisterPage } from "./pages/auth/Register.jsx"
import { AccountValidation } from "./pages/auth/AccountValidation.jsx"

import { Toast } from "./components/ui/Toast.jsx"
import { MainLayout } from "./layouts/MainLayout.jsx"
import { UserProvider } from "./context/userContext.jsx"
import { AuthProvider } from "./context/authContext.jsx"
import { ProtectedRoute } from "./router.jsx"

const root = ReactDOM.createRoot(document.getElementById("root"))

root.render(

    <>
        <AuthProvider>
            <UserProvider>

                <BrowserRouter>
                    <Routes>

                        <Route path="/" element={<MainLayout><Home /></MainLayout>} />
                        <Route path="/auth/login" element={<LoginPage />} />
                        <Route path="/auth/register" element={<RegisterPage />} />
                        <Route path="/auth/validate/:id/:token" element={<AccountValidation />} />
                        <Route path="/users/change-email/:id/:token" element={<EmailUpdate />} />

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

    </>
)
