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
