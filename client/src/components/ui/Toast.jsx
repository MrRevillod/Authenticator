
import { Toaster } from "sonner"
import { useEffect, useState } from "react"

export const Toast = () => {

    const [windowWidth, setWindowWidth] = useState(window.innerWidth)

    useEffect(() => {

        const handleResize = () => {
            setWindowWidth(window.innerWidth)
        }

        window.addEventListener("resize", handleResize)

        return () => {
            window.removeEventListener("resize", handleResize)
        }
    
    }, [])

    return (
        <Toaster position={windowWidth >= 768 ? "top-right" : "top-right"} />
    )
}