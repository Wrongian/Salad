import { blackSwanError } from "../../stores/stores";
const BLACKSWAN_ERROR_STATUS_CODE = 500;
const BAD_REQUEST_STATUS = 400;
import { validatePayload, type Validator } from "./validator.js";
import { TErrorValidator, type TError } from "./validation/error.js";
import {
  TStandardResponsePayloadValidator,
  type TStandardResponsePayload,
} from "./validation/response.js";
import { addError } from "$lib/modules/Errors.svelte";

const MASKED_ERROR_MESSAGE =
  "Oh no! Looks like something went wrong. Please try again later.";

type HttpMethods =
  | "GET"
  | "POST"
  | "PUT"
  | "PATCH"
  | "UPDATE"
  | "DELETE"
  | "HEAD";
type fetch = typeof fetch;

export async function validateFetch<R, U extends object = {}>(
  endpoint: string,
  // the HTTP
  method: HttpMethods,
  // The request payload
  payload: U,
  // Validator for the response
  validator: Validator<R>,
  // optional options
  options?: {
    // whether to use your own fetch or the universal fetch
    fetch?: fetch;
    // whether the body is a blob or not
    isBlobBody?: boolean;
  },
): Promise<R | null> {
  const hasBody = !(method == "GET" || method == "HEAD");
  const isBlobBody =
    hasBody && Boolean(options?.isBlobBody) && payload instanceof Blob;
  const useFetch = options?.fetch ?? fetch;
  // check body
  const request = hasBody
    ? {
        method,
        // Type cast here is safe because isBlobBody => payload instanceof Blob
        body: isBlobBody ? (payload as Blob) : JSON.stringify(payload),
      }
    : {
        method,
      };
  // get the response
  const response = await useFetch(endpoint, request);

  //
  const jsonBody = await response.json().catch((_) => {
    // black swan
    blackSwanError.set({
      status: BLACKSWAN_ERROR_STATUS_CODE,
      message: MASKED_ERROR_MESSAGE,
    });
    return null;
  });

  if (!response.ok) {
    return await validatePayload<TError>(jsonBody, TErrorValidator)
      .then(({ err }) => {
        if (response.status === 400) {
          addError(err, response.status);
        } else if (response.status === 403) {
          // forbidden page
        } else if (response.status === 404) {
          // not found
        } else {
          blackSwanError.set({
            status: BLACKSWAN_ERROR_STATUS_CODE,
            message: MASKED_ERROR_MESSAGE,
          });
        }
        return null;
      })
      .catch((_) => {
        blackSwanError.set({
          status: BLACKSWAN_ERROR_STATUS_CODE,
          message: MASKED_ERROR_MESSAGE,
        });
        return null;
      });
  }
  // { payload: T }
  if ("payload" in jsonBody) {
    // validate
    return await validatePayload(jsonBody.payload, validator).catch((_) => {
      blackSwanError.set({
        status: BLACKSWAN_ERROR_STATUS_CODE,
        message: MASKED_ERROR_MESSAGE,
      });
      return null;
    });
  } else {
    blackSwanError.set({
      status: BLACKSWAN_ERROR_STATUS_CODE,
      message: MASKED_ERROR_MESSAGE,
    });
    return null;
  }
}
