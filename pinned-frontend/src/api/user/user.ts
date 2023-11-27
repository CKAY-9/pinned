import axios from "axios"
import { API_URL } from "../resources";

export const getUserFromID = async (id: number) => {
  const user_request = await axios({
    "url": API_URL + "/users",
    "method": "GET",
    "params": {
      "id": id
    }
  });

  return user_request.data;
}

export const getUserFromToken = async (token: string = "") => {
  const user_request = await axios({
    "url": API_URL + "/users",
    "method": "GET",
    "headers": {
      "Authorization": token
    }
  });

  return user_request.data;
}
