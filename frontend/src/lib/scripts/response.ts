import type Joi from "joi"
import { TProfileBodyValidator } from "./response-validator"

// response
const handleResponseStatus = async (response: Response) => {
    if (response.ok) {
        // validate OK response (i.e. { payload })
    } 
    // validate Err response (i.e. { err })
}

