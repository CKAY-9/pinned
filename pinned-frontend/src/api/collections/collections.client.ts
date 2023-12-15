import axios from "axios";
import { API_URL } from "../resources";
import { getCookie } from "@/utils/cookies";

export const newCollection = async (name: string, description: string) => {
  try {
    const collection_request = await axios({
      "url": API_URL + "/collections",
      "method": "POST",
      "data": {
        "name": name,
        "description": description
      },
      "headers": {
        "Authorization": getCookie("token") || ""
      }
    });
    return collection_request.data.collection_id;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

export const getCollection = async (collection_id: number) => {
  try {
    const collection_request = await axios({
      "url": API_URL + "/collections",
      "method": "GET",
      "params": {
        "collection_id": collection_id
      }
    });
    return collection_request.data.collection;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}
