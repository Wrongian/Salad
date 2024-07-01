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

export type TAuthResult = {
  status: number;
  err: string;
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
