import type { RequestHandler } from "./$types";
const SERVER_IP_ADDR = import.meta.env.VITE_BACKEND_IP_ADDR;
// import { fetch } from 'undici'

// server-side reverse-proxy for all GET/POST/PUT/DELETE requests
export const fallback: RequestHandler = async ({
  request,
  params,
  url,
  fetch,
}) => {
  const tailURL = params.all + url.search;

  // remove headers to be updated by fetch
  request.headers.delete("content-length"); // handled by fetch API
  request.headers.delete("host"); // not needed; we already specify the url in fetch
  request.headers.delete("connection"); // handled by fetch API

  return await fetch(`${SERVER_IP_ADDR}/${tailURL}`, {
    body: request.body,
    method: request.method,
    headers: request.headers,
    redirect: "manual",
    // @ts-ignore
    // This is an edge case; duplex is a required property for forwarding the readablestream body
    // but is not typed by TS.
    duplex: "half",
  });
};
