
export const validateAccountTemplate = (url) => {

    return `

        <body>
            <table width="100%" bgcolor="#1A202C">
                <tr>
                    <td>
                        <h1 style="color: white; padding: 20px;">Workflow</h1>
                    </td>
                </tr>
            </table>
            <table width="100%">
                <tr>
                    <td>
                        <h2 style="color: black;">Validación de Cuenta</h2>
                        <p style="color: black;">¡Gracias por unirte a Workflow! Para validar tu cuenta, por favor haz clic en el siguiente enlace:</p>
                        <p><a href="${url}" style="color: #F5F5F5; background-color: #1A202C; padding: 8px 16px; text-decoration: none; border-radius: 4px;">Validar cuenta</a></p>
                        <p style="color: black;">Si no reconoces esta solicitud, puedes ignorar este mensaje.</p>
                        <p style="color: black;">Atentamente,</p>
                        <p style="color: black;">El equipo de Workflow</p>
                    </td>
                </tr>
                <tr>
                    <td>
                        <table width="100%" bgcolor="#1A202C">
                            <tr>
                                <td>
                                    <p style="color: white; text-align: center; padding: 10px;">© ${new Date().getFullYear()} Workflow</p>
                                </td>
                            </tr>
                        </table>
                    </td>
                </tr>
            </table>
        </body>
    `
}

export const changeEmailTemplate = (url) => {

    return `

        <body>
            <table width="100%" bgcolor="#1A202C">
                <tr>
                    <td>
                        <h1 style="color: white; padding: 20px;">Workflow</h1>
                    </td>
                </tr>
            </table>

            <table width="100%">
                <tr>
                    <td>
                        <h2 style="color: black;">Cambio de Correo Electrónico</h2>
                        <p style="color: black;">¡Gracias por unirte a Workflow! Para cambiar tu correo electrónico, por favor haz clic en el siguiente enlace:</p>
                        <p><a href="${url}" style="color: #F5F5F5; background-color: #1A202C; padding: 8px 16px; text-decoration: none; border-radius: 4px;">Cambiar correo electrónico</a></p>
                        <p style="color: black;">Si no reconoces esta solicitud, puedes ignorar este mensaje.</p>
                        <p style="color: black;">Atentamente,</p>
                        <p style="color: black;">El equipo de Workflow</p>
                    </td>
                </tr>
                <tr>
                    <td>
                        <table width="100%" bgcolor="#1A202C">
                            <tr>
                                <td>
                                    <p style="color: white; text-align: center; padding: 10px;">© ${new Date().getFullYear()} Workflow</p>
                                </td>
                            </tr>
                        </table>
                    </td>
                </tr>
    `
}

export const resetPasswordTemplate = (url) => {

    return `

        <body>
            <table width="100%" bgcolor="#1A202C">
                <tr>
                    <td>
                        <h1 style="color: white; padding: 20px;">Workflow</h1>
                    </td>
                </tr>
            </table>

            <table width="100%">
                <tr>
                    <td>
                        <h2 style="color: black;">Restauración de Contraseña</h2>
                        <p style="color: black;">Hemos recibido una solicitud para restaurar tu contraseña. Para hacerlo, por favor haz clic en el siguiente enlace:</p>
                        <p><a href="${url}" style="color: #F5F5F5; background-color: #1A202C; padding: 8px 16px; text-decoration: none; border-radius: 4px;">Restaurar Contraseña</a></p>
                        <p style="color: black;">Si no reconoces esta solicitud, puedes ignorar este mensaje.</p>
                        <p style="color: black;">Atentamente,</p>
                        <p style="color: black;">El equipo de Workflow</p>
                    </td>
                </tr>
                <tr>
                    <td>
                        <table width="100%" bgcolor="#1A202C">
                            <tr>
                                <td>
                                    <p style="color: white; text-align: center; padding: 10px;">© ${new Date().getFullYear()} Workflow</p>
                                </td>
                            </tr>
                        </table>
                    </td>
                </tr>
    `
}