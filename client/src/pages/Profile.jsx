
import { ProfileForm } from "../components/ProfileForm.jsx"
import { useUserStore } from "../lib/store.js"

import "../index.css"

export const Profile = () => {

    const user = useUserStore(state => state.user)

    if (!user) return null

    return (

        <div className="w-full h-full flex flex-col items-center text-neutral-200">

            <div className="w-full h-full md:px-8 py-4 flex flex-col items-center gap-12">

                <h2 className="text-2xl md:text-4xl">{user.name}</h2>
                <h4 className="text-lg md:text-xl text-neutral-300">@{user.username}</h4>

                <ProfileForm />

            </div>

        </div>
    )
}
