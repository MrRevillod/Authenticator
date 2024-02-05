
import { useState } from "react"
import { useAuth } from "../context/useAuth.jsx"

import "../index.css"

export const Profile = () => {

    const { user } = useAuth()

    let portada = {
        backgroundImage: `url(/portada.jpg)`,
        backgroundSize: "cover",
        backgroundPosition: "center",
        objectFit: "contain",
    }

    return (

        <div className="w-full h-full flex flex-col items-center text-neutral-200">

            <div className="h-1/5 md:h-1/4 w-full rounded-lg" style={portada}></div>
            <div className="absolute mt-16 md:mt-24">
                <img src="/default-profile.png" className="rounded-full md:w-52 md:h-52 w-40 h-40" />
            </div>

            <div className="w-full h-3/4 md:px-8 py-4 flex flex-col items-center gap-8 mt-24 md:mt-12">

                <div className="hidden md:flex justify-end w-full h-12 items-center text-neutral-100">
                    <button className="bg-neutral-200 rounded-lg flex justify-center items-center py-2 px-6">
                        <p className="font-semibold text-neutral-950">Editar perfíl</p>
                    </button>
                </div>

                <div className="flex flex-col w-full justify-center items-center gap-2 md:gap-4">
                    <h2 className="text-2xl md:text-4xl">{user.name}</h2>
                    <h4 className="text-lg md:text-xl text-neutral-300">@{user.username}</h4>
                </div>

                <div className="flex md:hidden justify-center w-full h-12 items-center text-neutral-100">
                    <button className="w-full bg-neutral-200 rounded-lg flex justify-center items-center py-2 px-6">
                        <p className="font-semibold text-neutral-950">Editar perfíl</p>
                    </button>
                </div>

                <div className="flex flex-col w-11/12 md:w-5/6 lg:w-2/3 justify-center items-center gap-2 md:gap-4">
                    <h2 className="text-xl md:text-2xl">Biografía / Sobre mi</h2>
                    <p className="text-sm md:text-base text-neutral-300 text-center">
                        Lorem ipsum dolor sit amet consectetur adipisicing elit. Debitis sunt reiciendis iure voluptas
                        nisi possimus quidem. Cumque quia corrupti inventore ipsam culpa labore dicta velit,
                        fuga quae nesciunt, repellat veritatis eum quos iusto, nobis voluptatem eius sint dignissimos
                        quam! At totam inventore qui? Non, quas exercitationem ducimus mollitia asperiores eligendi!
                    </p>
                </div>

            </div>

        </div>
    )
}
