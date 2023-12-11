import axios from "axios";
import { API_URL } from "../resources";
import { getCookie } from "@/utils/cookies";
import { NewCommentResponseDTO } from "./dto";

export const createComment = async (
  content: string,
  post: number
): Promise<NewCommentResponseDTO | null> => {
  try {
    const comment_request = await axios({
      "url": API_URL + "/comments",
      "method": "POST",
      "data": {
        "content": content,
        "post": post
      },
      "headers": {
        "Authorization": getCookie("token") || ""
      }
    });
    return comment_request.data;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

export const getCommentFromID = async (comment_id: number): Promise<Comment | null> => {
  try {
    const comment_request = await axios({
      "url": API_URL + "/comments",
      "method": "GET",
      "params": {
        "comment_id": comment_id
      }
    });
    return comment_request.data;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}
