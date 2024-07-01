import { TLink } from "$lib/scripts/response-validator";

export type LinkData = {
  id: number;
  title: string;
  description: string;
  imageLink: string;
  url: string;
};

export type LinkArray = {
  links: Array<LinkArray>;
};
export type ListData = {
  linkData: TLink;
  isDragged: boolean;
};
