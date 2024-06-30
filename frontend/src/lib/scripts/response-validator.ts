import Joi from "joi";
import type { TResponseBody } from "./query";

export const standardResponseValidator = (res: any): res is TResponseBody => {
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
  display_name: Joi.string().min(0).required(),
  bio: Joi.string().allow(null).min(0),
  picture: Joi.string().min(0),
  following: Joi.number().optional(),
  followers: Joi.number().optional(),
  is_private: Joi.boolean(),
}).unknown();

export type TLink = {
  id: number;
  user_id: number;
  next_id: number | null 
  title: string | null;
  href: string;
  description: string | null;
  img_src: string | null;
}

export const TLinkBodyValidator = Joi.object<{links: TLink[]}>({
  links: Joi.array<TLink[]>().items(
          Joi.object({
            id: Joi.number(),
            user_id: Joi.number().required(),
            next_id: Joi.number().allow(null).optional(),
            href: Joi.string().min(0).required(),
            title: Joi.string().min(0).allow(null).optional(),
            description: Joi.string().min(0).allow(null).optional(),
            img_src: Joi.string().allow(null).optional(),
          })
        ).min(0)
});
