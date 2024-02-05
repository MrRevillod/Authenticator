
import { Navbar } from "../components/Navbar.jsx"

export const MainLayout = ({ children }) => {

    return (

        <div className="h-screen w-screen bg-neutral-950 flex flex-col justify-start items-center">

            <Navbar />

            <main className="
                w-full flex-grow items-center justify-center pt-28 pb-12 px-6 md:px-12 lg:px-28 
            ">

                <div className="w-full h-full">

                    {children}

                </div>

            </main>

        </div>
    )
}