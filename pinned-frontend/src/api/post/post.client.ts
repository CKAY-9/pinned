import axios, { AxiosResponse } from "axios";
import { uploadFile } from "../cdn/cdn.client";
import { API_URL } from "../resources";
import { getCookie } from "@/utils/cookies";
import { NewPostResponseDTO, Post } from "./dto";

export const newPost = async (
  title: string, 
  description: string, 
  upload_file: File | null, 
  user_id: number
): Promise<Post | null> => {
  try {
    let dest = "";
    if (upload_file !== null) {
      const dest_response = await uploadFile(upload_file, {
        "folder_id": `user_${user_id}`,
        "previous_file_dest": ""
      });
      if (dest_response === null) {
        return null;
      }
      dest = dest_response.dest;
    }

    const post_request: AxiosResponse<NewPostResponseDTO> = await axios({
      "url": API_URL + "/posts",
      "method": "POST",
      "data": {
        "title": title,
        "description": description,
        "file_id": dest || "",
      },
      "headers": {
        "Authorization": getCookie("token") || ""
      }
    });

    return post_request.data.post;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

export const updatePost = async (post_id: number, title: string, description: string) => {
  try {
    const update_request = await axios({
      "url": API_URL + "/posts",
      "method": "PUT",
      "data": {
        "title": title,
        "description": description,
        "post_id": post_id
      },
      "headers": {
        "Authorization": getCookie("token") || ""
      }
    });
    return update_request.data;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

export const deletePost = async (post_id: number) => {
  try {
    const delete_request = await axios({
      "url": API_URL + "/posts",
      "method": "DELETE",
      "data": {
        "post_id": post_id
      },
      "headers": {
        "Authorization": getCookie("token") || ""
      }
    });

    return delete_request.data;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

export const likePost = async (like_type: number, post_id: number) => {
  try {
    const like_request = await axios({
      "url": API_URL + "/posts/like",
      "method": "PUT",
      "data": {
        "post_id": post_id,
        "like_type": like_type
      },
      "headers": {
        "Authorization": getCookie("token") || ""
      }
    });
    return like_request.data;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

export const searchPosts = async (name: string, id: number): Promise<Post[]> => {
  try {
    const search_request = await axios({
      "url": API_URL + "/posts/search",
      "method": "GET",
      "params": {
        "name": name,
        "post_id": id
      }
    });
    return search_request.data.posts;
  } catch (ex) {
    console.log(ex);
    return [];
  }
}

export const getExplorePosts = async (): Promise<Post[]> => {
  try {
    const explore_request = await axios({
      "url": API_URL + "/posts/explore",
      "method": "GET"
    });
    return explore_request.data.posts;
  } catch (ex) {
    console.log(ex);
    return []
  }
}

export const getPinnedPosts = async (): Promise<Post[]> => {
  try {
    const explore_request = await axios({
      "url": API_URL + "/posts/pinned",
      "method": "GET"
    });
    return explore_request.data.posts;
  } catch (ex) {
    console.log(ex);
    return []
  }
}

export const favouritePost = async (post_id: number): Promise<boolean> => {
  try {
    const request = await axios({
      url: API_URL + "/posts/favourite",
      method: "POST",
      data: {
        post_id: post_id
      },
      headers: {
        Authorization: getCookie("token") || ""
      }
    });
    return true;
  } catch (ex) {
    console.log(ex);
    return false;
  }
}