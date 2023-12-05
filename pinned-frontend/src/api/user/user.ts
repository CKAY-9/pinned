import axios, { AxiosResponse } from "axios"
import { API_URL } from "../resources";
import { User } from "./dto";
import { cookies } from "next/headers";

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

export const getUserFromToken = async (token: string = ""): Promise<null | User> => {
  try {
    if (token === "") {
      let temp_token = cookies().get("token")?.value; 
      if (temp_token === undefined)
        return null;
      token = temp_token;
    }

    const user_request: AxiosResponse<{message: string, user: User}> = await axios({
      "url": API_URL + "/users",
      "method": "GET",
      "headers": {
        "Authorization": token
      }
    });

    return user_request.data.user;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}
