// callback function
export type Callback = () => void;
export type ImageSubmitFunction = (image: Blob, filetype: string) => Promise<void>;