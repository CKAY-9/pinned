import axios, { AxiosResponse } from "axios";
import { User } from "./dto";
import { API_URL } from "../resources";
import { getCookie, setCookie } from "@/utils/cookies";

export const searchUsers = async (username: string = "", id: number = 0): Promise<User[]> => {
  try {
    const users_request = await axios({
      "url": API_URL + "/users/search",
      "method": "GET",
      "params": {
        "username": username,
        "id": id
      }
    });
    return users_request.data.users; 
  } catch (ex) {
    console.log(ex);
    return [];
  }
}

export const getUserPosts = async (user_id: number) => {
  try {
    const posts_request = await axios({
      "url": API_URL + "/users/posts",
      "method": "GET",
      "params": {
        "user_id": user_id
      }
    });
    return posts_request.data.posts;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

export const getUserCollections = async (user_id: number) => {
  try {
    const posts_request = await axios({
      "url": API_URL + "/users/collections",
      "method": "GET",
      "params": {
        "user_id": user_id
      }
    });
    return posts_request.data.collections;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

export const getUserComments = async (user_id: number) => {
  try {
    const posts_request = await axios({
      "url": API_URL + "/users/comments",
      "method": "GET",
      "params": {
        "user_id": user_id
      }
    });
    return posts_request.data.comments;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

export const getUserFromID = async (id: number): Promise<null | User> => {
  try {
    const user_request: AxiosResponse<{message: string, user: User}> = await axios({
      "url": API_URL + "/users/public",
      "method": "GET",
      "params": {
        "id": id
      }
    });

    return user_request.data.user;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

export const deleteUser = async () => {
  try {
    const delete_request = await axios({
      "url": API_URL + "/users",
      "method": "DELETE",
      "headers": {
        "Authorization": getCookie("token") || ""
      }
    });
    setCookie("token", "", 0);
    window.location.href = "/user/login";
  } catch (ex) {
    console.log(ex);
  }
}

export const logoutUser = () => {
  setCookie("token", "", 0);
  window.location.href = "/user/login";
}

export const resetUser = async () => {
  try {
    const reset_request = await axios({
      "url": API_URL + "/users/reset",
      "method": "POST",
      "headers": {
        "Authorization": getCookie("token") || ""
      }
    });
    window.location.reload();
  } catch (ex) {
    console.log(ex);
  }
}

export const getExploreUsers = async (): Promise<User[]> => {
  try {
    const request = await axios({
      url: API_URL + "/users/explore",
      method: "GET"
    });

    return request.data;
  } catch (ex) {
    console.log(ex);
    return []
  }
}