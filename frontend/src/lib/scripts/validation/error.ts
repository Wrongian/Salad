import Joi from "joi"

export type TError = { err: string }

export const TErrorValidator = Joi.object<TError>({
  err: Joi.string().min(0)
})