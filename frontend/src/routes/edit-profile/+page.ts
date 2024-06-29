import { redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { getIsLoggedIn } from "$lib/scripts/queries";
/**
 * validates and prepares the corresponding page data
 * @param param0
 * @returns an object to be pointed to by 'data' variable in +page.svelte
 */
export const load: PageLoad = async ({ data, route, fetch, params } : any) => {
  // change later
  // probably use svelte store for both isloggedin and the prev routing
  const isLoggedIn = await getIsLoggedIn(fetch);
  if (!isLoggedIn){
    redirect(302,"/auth/login");
  }
  // placeholder
  return {
    displayName : "Clown",
    bio: "We live in a clown society",
    imageURL: "https://media.istockphoto.com/id/533837393/photo/clown.jpg?s=1024x1024&w=is&k=20&c=tSlAog5TTEeuG-c4hfBIKwo2X2Cy03l3r8LIF0HFWUE=",
    links: [
      {
        id: 1,
        title: "Title 1",
        description:"This is the bio of all time 1", 
        imageLink:"https://th-thumbnailer.cdn-si-edu.com/6RD8JDrASGTSjdbsJkg-37OY9mQ=/1072x720/filters:no_upscale()/https://tf-cmsv2-smithsonianmag-media.s3.amazonaws.com/filer/d5/24/d5243019-e0fc-4b3c-8cdb-48e22f38bff2/istock-183380744.jpg",
        url: "www.life.org",
      },
      {
        id: 2,
        title: "Title 2",
        description:"This is the bio of all time 2", 
        imageLink:"https://th-thumbnailer.cdn-si-edu.com/6RD8JDrASGTSjdbsJkg-37OY9mQ=/1072x720/filters:no_upscale()/https://tf-cmsv2-smithsonianmag-media.s3.amazonaws.com/filer/d5/24/d5243019-e0fc-4b3c-8cdb-48e22f38bff2/istock-183380744.jpg",
        url: "www.life.org",
      },
      {
        id: 3,
        title: "Title 3",
        description:"this is the bio of all time 3", 
        imageLink:"https://th-thumbnailer.cdn-si-edu.com/6rd8jdrasgtsjdbsjkg-37oy9mq=/1072x720/filters:no_upscale()/https://tf-cmsv2-smithsonianmag-media.s3.amazonaws.com/filer/d5/24/d5243019-e0fc-4b3c-8cdb-48e22f38bff2/istock-183380744.jpg",
        url: "www.life.org",
      },
      {
        id: 4,
        title: "Title 4",
        description:"this is the bio of all time 4", 
        imageLink:"https://th-thumbnailer.cdn-si-edu.com/6rd8jdrasgtsjdbsjkg-37oy9mq=/1072x720/filters:no_upscale()/https://tf-cmsv2-smithsonianmag-media.s3.amazonaws.com/filer/d5/24/d5243019-e0fc-4b3c-8cdb-48e22f38bff2/istock-183380744.jpg",
        url: "www.life.org",
      },
      ],
    };
};


