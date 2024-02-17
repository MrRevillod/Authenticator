
import { connect } from "mongoose"
import { DB_NAME, DB_URI } from "./config.js"

export const connectDatabase = async () => {

    try { return await connect(DB_URI, { dbName: DB_NAME }) }

    catch (error) {
        throw { status: 500, message: "Internal server error" }
    }
}