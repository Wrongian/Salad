import Joi from "joi";
import type { TResponseBody } from "./query";

export const authResponseValidator = (res: any): res is TResponseBody => {
  if (!("result" in res) || !("err" in res)) return false;
  return typeof res.result === "boolean" && typeof res.err === "string";
};

export type TProfileBody = {
  display_name: string;
  bio: string;
  picture: string;
  following: number | null;
  followers: number | null;
  is_private: boolean;
};

export const TProfileBodyValidator = Joi.object<TProfileBody>({
  display_name: Joi.string().required(),
  bio: Joi.string().min(0),
  picture: Joi.string().min(0),
  following: Joi.number().optional(),
  followers: Joi.number().optional(),
  is_private: Joi.boolean(),
}).unknown();

export type TLinkBody = Array<{
  link_id: string;
  title: string;
  href: string;
  description: string;
  picture: string;
}>;

export const TLinkBodyValidator = Joi.array<TLinkBody>().items(
  Joi.object({
    link_id: Joi.string(),
    title: Joi.string(),
    href: Joi.string(),
    description: Joi.string(),
    picture: Joi.string(),
  })
);
