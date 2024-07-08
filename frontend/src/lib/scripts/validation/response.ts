import Joi from "joi"

export type TStandardPayload = {}

export const TStandardPayloadValidator = Joi.object<TStandardPayload>()