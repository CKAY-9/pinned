import axios from "axios";
import { API_URL } from "../resources";
import { Post } from "./dto";

export const getPostFromID = async (post_id: number): Promise<Post | null> => {
  try {
    const post_request = await axios({
      "url": API_URL + "/posts",
      "method": "GET",
      "params": {
        "post_id": post_id
      }
    });
    return post_request.data.post;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}
