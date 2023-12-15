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

export const addToCollection = async (collection_id: number, post_id: number) => {
  try {
    const add_request = await axios({
      "url": API_URL + "/collections/add",
      "method": "PUT",
      "data": {
        "collection_id": collection_id,
        "post_id": post_id
      },
      "headers": {
        "Authorization": getCookie("token") || ""
      }
    });
    return add_request.data;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

export const deleteCollection = async (collection_id: number) => {
  try {
    const delete_request = await axios({
      "url": API_URL + "/collections",
      "method": "DELETE",
      "data": {
        "collection_id": collection_id
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

export const updateCollection = async (collection_id: number, name: string, description: string) => {
  try {
    const update_request = await axios({
      "url": API_URL + "/collections",
      "method": "PUT",
      "data": {
        "collection_id": collection_id,
        "name": name,
        "description": description
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
