
import React from "react"
import ReactDOM from "react-dom/client"

import './index.css'

import { HomePage } from "./pages/Home.jsx"
import { DashboardPage } from "./pages/Dashboard.jsx"

import { LoginPage } from "./pages/auth/Login.jsx"
import { RegisterPage } from "./pages/auth/Register.jsx"

import { ProfilePage } from "./pages/account/Profile.jsx"
import { EmailUpdatePage } from "./pages/account/EmailUpdate.jsx"
import { AccountValidationPage } from "./pages/account/AccountValidation.jsx"

import { Toast } from "./components/ui/Toast.jsx"
import { MainLayout } from "./layouts/MainLayout.jsx"
import { UserProvider } from "./context/userContext.jsx"
import { AuthProvider } from "./context/authContext.jsx"
import { ProtectedRoute } from "./router.jsx"
import { BrowserRouter, Routes, Route } from "react-router-dom"
import { ForgotPasswordPage, ForgotPasswordRequestPage } from "./pages/auth/ResetPassword.jsx"

const root = ReactDOM.createRoot(document.getElementById("root"))

root.render(

    <>
        <AuthProvider>
            <UserProvider>
                <BrowserRouter>
                    <Routes>

                        <Route path="/" element={<MainLayout><HomePage /></MainLayout>} />
                        <Route path="/auth/login" element={<LoginPage />} />
                        <Route path="/auth/register" element={<RegisterPage />} />
                        <Route path="/auth/reset-password" element={<MainLayout><ForgotPasswordRequestPage /></MainLayout>} />
                        <Route path="/auth/reset-password/:id/:token" element={<MainLayout> <ForgotPasswordPage /></MainLayout>} />

                        <Route path="/account/validate/:id/:token" element={<AccountValidationPage />} />
                        <Route path="/account/update-email/:id/:token" element={<EmailUpdatePage />} />

                        <Route element={<ProtectedRoute />}>

                            <Route path="/dashboard" element={
                                <MainLayout>
                                    <DashboardPage />
                                </MainLayout>
                            } />

                            <Route path="/account/profile" element={
                                <MainLayout>
                                    <ProfilePage />
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
