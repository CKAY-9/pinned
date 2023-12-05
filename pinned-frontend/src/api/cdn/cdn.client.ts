import axios, { AxiosResponse } from "axios";
import { CDN_URL } from "../resources";
import { UploadFileResponse } from "./dto";

export const uploadFile = async (file_data: File | undefined, data: {
    folder_id: string,
    previous_file_dest: string
}): Promise<UploadFileResponse | null> => {
  try {
    if (file_data === undefined) {
      return null;
    }

    if (file_data.size > (1024 * 1024 * 3) /* 3MB */) {
      return null;
    }

    const form = new FormData();
    form.append("folder_id", data.folder_id);
    form.append("previous_file", data.previous_file_dest)
    form.append("file", file_data);

    const req: AxiosResponse<UploadFileResponse> = await axios({
      "url": CDN_URL + "/upload",
      "method": "POST",
      "data": form  
    });

    return req.data;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}
