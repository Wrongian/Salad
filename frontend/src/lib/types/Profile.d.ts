export type LinkData = {
    id: number,
    title: string,
    description: string,
    imageLink: string,
    url: string,
}

export type LinkArray = {
    links : Array<LinkArray>,
}
export type ListData = {
    linkData : LinkData,
    isDragged : boolean,
}