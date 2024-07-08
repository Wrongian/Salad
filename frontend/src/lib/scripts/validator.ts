
import type Joi from "joi"

export type Validator<T> = Joi.Schema<T>
// response later
export const validatePayload = async <T>(payload: unknown, validator: Validator<T>) : Promise<T> => {
    return validator.validateAsync(payload)
}