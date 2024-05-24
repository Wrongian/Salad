import type { TResponseBody } from "./query";

export const authResponseValidator = (res: any): res is TResponseBody => {
    if (!('result' in res) || !('err' in res)) return false
    return typeof res.result === "boolean" && typeof res.err === "string";
}
