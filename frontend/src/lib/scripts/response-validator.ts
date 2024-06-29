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

export type TLink = {
  id: number;
  user_id: number;
  next_id: number | undefined
  title: string | undefined;
  href: string;
  description: string | undefined;
  img_src: string | undefined;
}

export const TLinkBodyValidator = Joi.object<{links: TLink[]}>({
  links: Joi.array<TLink[]>().items(
          Joi.object({
            id: Joi.number(),
            user_id: Joi.number().required(),
            next_id: Joi.number().optional(),
            href: Joi.string().required(),
            title: Joi.string().optional(),
            description: Joi.string().optional(),
            img_src: Joi.string().optional(),
          })
        ).min(0)
});
