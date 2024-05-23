export type TRenderErrorProp = {
    id: number,
    statusCode: number,
    message: string
}

export type TErrorContext = {
    addError: (message: string, statusCode: number) => void,
    removeAt: (index: number) => void
}