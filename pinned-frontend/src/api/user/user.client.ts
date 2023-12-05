import axios, { AxiosResponse } from "axios";
import { User } from "./dto";
import { API_URL } from "../resources";
import { getCookie, setCookie } from "@/utils/cookies";

export const searchUsers = async (username: string = "", id: number = 0): Promise<User[]> => {
  try {
    const users_request: AxiosResponse<User[]> = await axios({
      "url": API_URL + "/users/search",
      "method": "GET",
      "params": {
        "username": username,
        "id": id
      }
    });

    return users_request.data || [];
  } catch (ex) {
    console.log(ex);
    return [];
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
