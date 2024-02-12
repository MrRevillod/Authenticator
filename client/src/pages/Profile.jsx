
import { useUser } from "../context/userContext.jsx"
import { Spinner } from "../components/ui/Spinner.jsx"
import { ProfileForm } from "../components/ProfileForm.jsx"

import "../index.css"

export const Profile = () => {

    const { isLoading } = useUser()

    return (

        <div className="w-full h-full flex flex-col items-center text-neutral-200">

            {isLoading && (<Spinner classes={"z-10 fixed opacity-100"} />)}

            <div className="w-full h-full md:px-8 py-4 flex flex-col items-center gap-12">

                {user && (
                    <>
                        <h2 className="text-2xl md:text-4xl">{user.name}</h2>
                        <h4 className="text-lg md:text-xl text-neutral-300">@{user.username}</h4>
                    </>
                )}

                <ProfileForm />

            </div>

        </div>
    )
}
