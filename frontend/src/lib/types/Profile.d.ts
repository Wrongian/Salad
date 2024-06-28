export type LinkData = {
    id: number,
    title: string,
    bio: string,
    imageLink: string,
}

export type LinkArray = {
    links : Array<LinkArray>,
}
export type ListData = {
    linkData : LinkData,
    isDragged : boolean,
}