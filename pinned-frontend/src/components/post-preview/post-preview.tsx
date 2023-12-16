import Link from "next/link";
import Image from "next/image";
import style from "./post-preview.module.scss";
import { Post } from "@/api/post/dto";
import { CDN_URL } from "@/api/resources";

const PostPreview = (props: {
  post: Post
}) => {
  return (
    <>
      <Link className={style.post} href={`/post/${props.post.id}`}>
        <h1>{props.post.title}</h1> 
        {props.post.file_id.length >= 1 &&
          <Image 
            src={CDN_URL + props.post.file_id}
            alt="Post Picture"
            sizes="100%"
            width={0}
            height={0}
          />
        }
        <span>{props.post.description.substring(0, 50)}</span>
      </Link>
    </>
  );
}

export default PostPreview;
