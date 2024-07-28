export type TUpdateProfileQuery = {
  username: string;
  name?: string;
  bio?: string;
  is_private?: boolean;
};

export type TUpdateProfile = {
  display_name?: string;
  bio?: string;
};

export type TCreateLinkPayload = {
  title?: string;
  bio?: string;
  href: string;
};

export type TUpdateLinkTitlePayload = {
  title: string;
};

export type TUpdateLinkBioPayload = {
  bio: string;
};

export type TUpdateLinkHrefPayload = {
  href: string;
};

export type TReorderPayload = {
  link_id: number;
  new_position_id: number | null;
};

export type TCreateFollowRequestPayload = {
  pending_follow_id: number;
}

export type TCompleteFollowRequestPayload = {
  from_id: number;
  accept: boolean
}

export type TResponseBody = {
  result: boolean;
  err: string;
};

export type TResult<T> =
  | {
    payload: T;
    success: true;
  }
  | {
    success: false;
    status: number;
    err: string;
  };

export type TGetEmailBody = {
  email: string;
};
export type TResetCodeBody = {
  email: string;
  code: string;
};

export type TResetPasswordBody = {
  email: string;
  code: string;
  password: string;
};

