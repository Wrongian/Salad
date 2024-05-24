export type TUpdateProfileQuery = {
    username: string
    name?: string,
    bio?: string,
    is_private?: boolean
}

export type TAuthResult = {
    status: number;
    err: string;
}


export type TResponseBody = {
    result: boolean,
    err: string
}