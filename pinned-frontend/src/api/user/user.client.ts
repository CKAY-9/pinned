import axios, { AxiosResponse } from "axios";
import { User } from "./dto";
import { API_URL } from "../resources";

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

    console.log(users_request.data);
    return users_request.data || [];
  } catch (ex) {
    console.log(ex);
    return [];
  }
}
