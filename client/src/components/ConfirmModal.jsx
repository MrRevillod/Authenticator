
export const ConfirmModal = ({ isOpen, text, onConfirm, onClose }) => {

    if (!isOpen) return null

    const handleConfirm = () => {
        onConfirm()
    }

    const handleCancel = () => {
        onClose()
    }

    return (
        <div className="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50">
            <div className="bg-neutral-950 border-1 border-neutral-600 p-6 rounded-lg">
                <p className="mb-4">{text}</p>
                <div className="flex justify-end space-x-2">
                    <button onClick={handleConfirm} className="px-4 py-2 bg-red-700 text-white rounded">Aceptar</button>
                    <button onClick={handleCancel} className="px-4 py-2 bg-gray-300 text-black rounded">Cancelar</button>
                </div>
            </div>
        </div>
    )
}