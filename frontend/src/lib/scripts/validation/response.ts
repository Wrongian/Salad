import Joi from "joi";

export type TStandardResponsePayload = {};

export const TStandardResponsePayloadValidator = Joi.object<TStandardResponsePayload>();

export type TResultPayload = { result: boolean };

export const TResultPayloadValidator = Joi.object<TResultPayload>({
  result: Joi.boolean(),
});

export type TGetUsernamePayload = { username: string | null };

export const TGetUsernamePayloadValidator = Joi.object<TGetUsernamePayload>({
  username: Joi.string().optional(),
});

export type TUpdateImageResponseBody = { href: string };

export const UpdateImageResponseBodyValidator =
  Joi.object<TUpdateImageResponseBody>({
    href: Joi.string().allow(""),
  });

// profile
export type TProfileBody = {
  display_name: string;
  bio: string;
  picture: string;
  following: number | null;
  followers: number | null;
  is_private: boolean;
  is_owner: boolean;
  id: number;
};

export const TProfileBodyValidator = Joi.object<TProfileBody>({
  display_name: Joi.string().min(0).required(),
  bio: Joi.string().allow(null).min(0),
  picture: Joi.string().min(0),
  following: Joi.number().optional(),
  followers: Joi.number().optional(),
  is_private: Joi.boolean(),
  is_owner: Joi.boolean(),
  id: Joi.number(),
}).unknown();

// link
export type TLink = {
  id: number;
  user_id: number;
  next_id: number | null;
  title: string | null;
  href: string;
  description: string | null;
  img_src: string | null;
};

export const TLinkBodyValidator = Joi.object<{ links: TLink[] }>({
  links: Joi.array<TLink[]>()
    .items(
      Joi.object({
        id: Joi.number(),
        user_id: Joi.number().required(),
        next_id: Joi.number().allow(null).optional(),
        href: Joi.string().min(0).required(),
        title: Joi.string().min(0).allow(null).optional(),
        description: Joi.string().min(0).allow(null).optional(),
        img_src: Joi.string().allow(null).optional(),
      }),
    )
    .min(0),
});

const FOLLOW_STATUSES = ['following', 'pending', 'none'] as const
export type FollowStatus = typeof FOLLOW_STATUSES[number]

export type TFollowStatusResponsePayload = {
  status: FollowStatus
}

export const TFollowStatusValidator = Joi.object<TFollowStatusResponsePayload>({
  status: Joi.string().valid(...FOLLOW_STATUSES)
})
