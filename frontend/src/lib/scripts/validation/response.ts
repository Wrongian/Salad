import Joi from "joi";

export type TStandardResponsePayload = {};

export const TStandardResponsePayloadValidator =
  Joi.object<TStandardResponsePayload>();

export type TResultPayload = { result: boolean };

export const TResultPayloadValidator = Joi.object<TResultPayload>({
  result: Joi.boolean(),
});

export type TGetUsernamePayload = { username: string | null };

export const TGetUsernamePayloadValidator = Joi.object<TGetUsernamePayload>({
  username: Joi.string().allow(null).allow(""),
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
  following: Joi.number().allow(null),
  followers: Joi.number().allow(null),
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

const FOLLOW_STATUSES = ["following", "pending", "none"] as const;
export type TFollowStatus = (typeof FOLLOW_STATUSES)[number];

export type TFollowStatusResponsePayload = {
  status: TFollowStatus;
};

export const TFollowStatusValidator = Joi.object<TFollowStatusResponsePayload>({
  status: Joi.string().valid(...FOLLOW_STATUSES),
});

export type TGetPaginatedProfilePayload<T extends object = TPaginatedProfile> =
  {
    profiles: T[];
    total_size: number;
  };

export type TPaginatedProfile = {
  username: string;
  img_src: string | undefined;
  id: number;
  display_name: string;
};

export const TGetPaginatedProfilePayloadValidator =
  Joi.object<TGetPaginatedProfilePayload>({
    profiles: Joi.array().items(
      Joi.object<TPaginatedProfile>({
        username: Joi.string().min(0),
        img_src: Joi.string().allow(null),
        id: Joi.number(),
        display_name: Joi.string().allow(null),
      }),
    ),
    total_size: Joi.number(),
  });

// type declarations for paginated pending follow requests
export const FOLLOW_REQUEST_TYPES = ["OUTGOING", "INCOMING"] as const;
export type TFollowRequest = (typeof FOLLOW_REQUEST_TYPES)[number];

export type TPaginatedFollowRequestProfile = TPaginatedProfile & {
  request_type: TFollowRequest;
};

export const TGetPaginatedFollowRequestProfileValidator = Joi.object<
  TGetPaginatedProfilePayload<TPaginatedFollowRequestProfile>
>({
  profiles: Joi.array().items(
    Joi.object<TPaginatedFollowRequestProfile>({
      username: Joi.string().min(0),
      img_src: Joi.string().allow(null),
      id: Joi.number(),
      display_name: Joi.string().allow(null),
      request_type: Joi.string().valid(...FOLLOW_REQUEST_TYPES),
    }),
  ),
  total_size: Joi.number(),
});

export type TUserInsightResponsePayload = {
  total_profile_views: number;
  interval_views: [Date, number][];
  interval_follows: [Date, number][];
  interval_unfollows: [Date, number][];
  interval_follow_requests: [Date, number][];
  interval_shares: [Date, number][];
};

export const UserInsightResponsePayloadValidator =
  Joi.object<TUserInsightResponsePayload>({
    total_profile_views: Joi.number(),

    interval_views: Joi.array().items(
      Joi.array().ordered(Joi.date().required(), Joi.number().required()),
    ),
    interval_follows: Joi.array().items(
      Joi.array().ordered(Joi.date().required(), Joi.number().required()),
    ),

    interval_unfollows: Joi.array().items(
      Joi.array().ordered(Joi.date().required(), Joi.number().required()),
    ),

    interval_follow_requests: Joi.array().items(
      Joi.array().ordered(Joi.date().required(), Joi.number().required()),
    ),

    interval_shares: Joi.array().items(
      Joi.array().ordered(Joi.date().required(), Joi.number().required()),
    ),
  });
