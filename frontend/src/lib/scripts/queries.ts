import type {
  TUpdateProfileQuery,
  TUpdateProfile,
  TCreateLinkPayload,
  TUpdateLinkTitlePayload,
  TUpdateLinkBioPayload,
  TUpdateLinkHrefPayload,
  TReorderPayload,
  TCreateFollowRequestPayload,
} from "./query.d.ts";
import { goto, invalidateAll } from "$app/navigation";
import {
  TFollowStatusValidator,
  TLinkBodyValidator,
  TProfileBodyValidator,
  UpdateImageResponseBodyValidator,
  type FollowStatus,
  type TFollowStatusResponsePayload,
  type TLink,
  type TProfileBody,
  type TUpdateImageResponseBody,
} from "./validation/response.js";
import {
  type TGetUsernamePayload,
  TGetUsernamePayloadValidator,
  TResultPayloadValidator,
  TStandardResponsePayloadValidator,
  type TResultPayload,
  type TStandardResponsePayload as TStandardResponsePayload,
} from "./validation/response.js";
import { validateFetch } from "./fetch.js";

const BASEURL = "/";
const PROFILES_PREFIX = "/api/profiles";
const UPDATE_PROFILE_IMAGE_ENDPOINT = PROFILES_PREFIX + "/image";
const UPDATE_DISPLAY_PROFILE_ENDPOINT = PROFILES_PREFIX + "/display";
const LINKS_PREFIX = "/api/links";
const GET_LINKS_ENDPOINT = "/api/links";
const ADD_LINKS_ENDPOINT = LINKS_PREFIX;
const UPDATE_LINK_TITLE_ENDPOINT = "/api/links/title";
const UPDATE_LINK_BIO_ENDPOINT = "/api/links/bio";
const UPDATE_LINK_HREF_ENDPOINT = "/api/links/href";
const REORDER_LINK_ENDPOINT = "/api/links/reorder";
const DELETE_LINK_ENDPOINT = "/api/links";
const LOGIN_ENDPOINT = "/api/login";
const LOGOUT_ENDPOINT = "/api/logout";
const REGISTER_ENDPOINT = "/api/register";
const GET_IS_LOGGED_IN_ENDPOINT = "/api/logged-in";
const GET_USERNAME_ENDPOINT = "/api/get-username";
const FOLLOW_REQUEST_ENDPOINT = "/api/follow-request";
const FOLLOW_STATUS_ENDPOINT = "/api/follow-status";
const FOLLOWER_ENDPOINT = "/api/follower";
const FOLLOWING_ENDPOINT = "/api/following";

type fetch = typeof fetch;

/**
 * forms a POST query to the /login endpoint to validate and log in user
 * expects: status 400 with message on error and 200 on successful login
 * expects: response with a body of type { result: boolean, token: string }
 * expects: if fetch promise is rejected, then response has the type { status: 400, message: error_message }
 * TODO: bearer/cookie based token + boolean result
 * @param username
 * @param password
 */
export const login = async (
  username: string,
  password: string,
  next: string,
): Promise<void> => {
  // validate request here
  const payload = await validateFetch<
    TStandardResponsePayload,
    { username: string; password: string }
  >(LOGIN_ENDPOINT, "POST", { username, password }, TStandardResponsePayloadValidator);

  if (payload !== null) {
    await invalidateAll();
  }
};

/**
 * forms a POST query to the /register endpoint to create a new user if it doesn't exist in the database.
 * expects: status 400 with message on error and 200
 * TODO: bearer/cookie based token + boolean result
 *
 * @param email
 * @param username
 * @param password
 */
export const register = async (
  email: string,
  username: string,
  password: string,
  next: string,
): Promise<void> => {
  // validate request here
  const payload = await validateFetch<
    TStandardResponsePayload,
    { username: string; password: string; email: string }
  >(
    REGISTER_ENDPOINT,
    "POST",
    { username, password, email },
    TStandardResponsePayloadValidator,
  );

  if (payload !== null) {
    await invalidateAll();
  }
};

export const updateProfile = async (updateQuery: TUpdateProfileQuery) => {
  await validateFetch<TStandardResponsePayload, TUpdateProfileQuery>(
    UPDATE_DISPLAY_PROFILE_ENDPOINT,
    "PUT",
    updateQuery,
    TStandardResponsePayloadValidator,
  );
};

export const getProfile = async (
  username: string,
  fetch: fetch,
): Promise<TProfileBody | null> => {
  return await validateFetch<TProfileBody, { username: string }>(
    `${PROFILES_PREFIX}/${username}`,
    "GET",
    { username },
    TProfileBodyValidator,
    { fetch },
  );
};

export const resetPassword = async (email: string) => {
  // TODO: finish up reset password implementation
  return email;
};

export const getLinks = async (
  username: string,
  fetch: fetch,
): Promise<TLink[]> => {
  // todo change the function
  return await validateFetch<{ links: TLink[] }>(
    `${GET_LINKS_ENDPOINT}/${username}`,
    "GET",
    {},
    TLinkBodyValidator,
    { fetch },
  ).then((linkBody) => {
    // return links if can
    if (linkBody) {
      return linkBody.links;
    }
    // return nothing
    return [];
  });
};

export const getFollowStatus = async (
  targetUserId: number,
  fetch: fetch
): Promise<FollowStatus | undefined> => {
  return await validateFetch<TFollowStatusResponsePayload>(
    `${FOLLOW_STATUS_ENDPOINT}?${new URLSearchParams([["id", targetUserId.toString()]]).toString()}`,
    "GET",
    {},
    TFollowStatusValidator,
    { fetch }
  ).then(payload => payload?.status)

}

export const getIsLoggedIn = async (fetch: fetch): Promise<boolean> => {
  return (
    (
      await validateFetch<TResultPayload>(
        GET_IS_LOGGED_IN_ENDPOINT,
        "GET",
        {},
        TResultPayloadValidator,
        { fetch },
      )
    )?.result ?? false
  );
};

// logout route, doesnt do anything if not logged in
// cant really get an error logging out since its a get request
export const logout = async (fetch: fetch, next: string): Promise<void> => {
  await validateFetch<TStandardResponsePayload>(
    LOGOUT_ENDPOINT,
    "GET",
    {},
    TStandardResponsePayloadValidator,
    { fetch },
  );

  await invalidateAll();
  if (next != null) {
    goto(next);
  } else {
    goto(BASEURL);
  }
};

// get username route
export const getUsername = async (fetch: fetch): Promise<string> => {
  const maybeUsername = await validateFetch<TGetUsernamePayload>(
    GET_USERNAME_ENDPOINT,
    "GET",
    {},
    TGetUsernamePayloadValidator,
    { fetch },
  );

  // we provide a default value to username in the event it is null
  // errors would have been handled by the frontend (via redirect)
  return maybeUsername?.username ?? "";
};

export const updateTextProfile = async (
  query: TUpdateProfile,
): Promise<void> => {
  let payload = await validateFetch<TStandardResponsePayload, TUpdateProfile>(
    UPDATE_DISPLAY_PROFILE_ENDPOINT,
    "PUT",
    query,
    TStandardResponsePayloadValidator,
  );

  if (payload !== null) {
    await invalidateAll();
  }
};

export const addLinks = async (query: TCreateLinkPayload): Promise<void> => {
  await validateFetch<TStandardResponsePayload, TCreateLinkPayload>(
    ADD_LINKS_ENDPOINT,
    "POST",
    query,
    TStandardResponsePayloadValidator,
  );
};

export const updateLinkTitle = async (
  query: TUpdateLinkTitlePayload,
  link_id: number,
) => {
  const payload = await validateFetch<
    TStandardResponsePayload,
    TUpdateLinkTitlePayload
  >(
    `${UPDATE_LINK_TITLE_ENDPOINT}/${link_id}`,
    "PUT",
    query,
    TStandardResponsePayloadValidator,
  );

  if (payload !== null) {
    await invalidateAll();
  }
};

export const updateLinkBio = async (
  query: TUpdateLinkBioPayload,
  link_id: number,
) => {
  const payload = await validateFetch<TStandardResponsePayload, TUpdateLinkBioPayload>(
    `${UPDATE_LINK_BIO_ENDPOINT}/${link_id}`,
    "PUT",
    query,
    TStandardResponsePayloadValidator,
  );

  if (payload !== null) {
    await invalidateAll();
  }
};

export const updateLinkHref = async (
  query: TUpdateLinkHrefPayload,
  link_id: number,
) => {
  const payload = await validateFetch<TStandardResponsePayload, TUpdateLinkHrefPayload>(
    `${UPDATE_LINK_HREF_ENDPOINT}/${link_id}`,
    "PUT",
    query,
    TStandardResponsePayloadValidator,
  );

  if (payload !== null) {
    await invalidateAll();
  }
};

export const deleteLink = async (link_id: number) => {
  const payload = await validateFetch<TStandardResponsePayload>(
    `${DELETE_LINK_ENDPOINT}/${link_id}`,
    "DELETE",
    {},
    TStandardResponsePayloadValidator,
  );

  if (payload !== null) {
    await invalidateAll();
  }
};

export const reorderLink = async (query: TReorderPayload) => {
  const payload = await validateFetch<TStandardResponsePayload, TReorderPayload>(
    REORDER_LINK_ENDPOINT,
    "POST",
    query,
    TStandardResponsePayloadValidator,
  );

  if (payload !== null) {
    await invalidateAll();
  }
};

export const updateLinkPicture = async (
  image: Blob,
  filetype: String,
  id: number,
): Promise<TUpdateImageResponseBody> => {
  console.log("image: ", image);
  return await validateFetch<TUpdateImageResponseBody, Blob>(
    `${LINKS_PREFIX}/${id.toString()}/image/${filetype}`,
    "PUT",
    image,
    UpdateImageResponseBodyValidator,
    { isBlobBody: true },
  ).then(async (payload) => {
    return payload ?? { href: "" };
  });
};

export const updateProfilePicture = async (
  image: Blob,
  filetype: String,
): Promise<TUpdateImageResponseBody> => {
  return await validateFetch<TUpdateImageResponseBody, Blob>(
    `${UPDATE_PROFILE_IMAGE_ENDPOINT}/${filetype}`,
    "PUT",
    image,
    UpdateImageResponseBodyValidator,
    { isBlobBody: true },
  ).then(async (payload) => {
    return payload ?? { href: "" };
  });
};

export const createFollowRequest = async (payload: TCreateFollowRequestPayload) => {
  return await validateFetch<TStandardResponsePayload>(
    `${FOLLOW_REQUEST_ENDPOINT}`,
    "POST",
    payload,
    TStandardResponsePayloadValidator
  )
}

export const removeFollower = async (userId: number) => {
  return await validateFetch<TStandardResponsePayload>(
    FOLLOWER_ENDPOINT,
    "DELETE",
    { follower_id: userId },
    TStandardResponsePayloadValidator
  )
}

export const removeFollowing = async (userId: number) => {
  return await validateFetch<TStandardResponsePayload>(
    FOLLOWING_ENDPOINT,
    "DELETE",
    { following_id: userId },
    TStandardResponsePayloadValidator
  )
}