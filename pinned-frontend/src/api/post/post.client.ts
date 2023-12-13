import axios from "axios";
import { uploadFile } from "../cdn/cdn.client";
import { API_URL } from "../resources";
import { getCookie } from "@/utils/cookies";
import { NewPostResponseDTO } from "./dto";

export const newPost = async (
  title: string, 
  description: string, 
  upload_file: File | null, 
  user_id: number
): Promise<NewPostResponseDTO | null> => {
  try {
    let dest = "";
    if (upload_file != null) {
      const dest_response = await uploadFile(upload_file, {
        "folder_id": `user_${user_id}`,
        "previous_file_dest": ""
      });
      if (dest_response !== null) {
        dest = dest_response.dest;
      }
    }

    const post_request = await axios({
      "url": API_URL + "/posts",
      "method": "POST",
      "data": {
        "title": title,
        "description": description,
        "file_dest": dest,
      },
      "headers": {
        "Authorization": getCookie("token") || ""
      }
    });

    return post_request.data;
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
      }
    });

    return delete_request.data;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}
