import { Post } from "@/api/post/dto";
import style from "./posts.module.scss";
import Link from "next/link";
import Image from "next/image";
import { CDN_URL } from "@/api/resources";

const UserPosts = (props: {
  posts: Post[] | null
}) => {
  if (props.posts === null) return;

  return (
    <div className={style.posts}>
      {props.posts.map((post: Post, index: number) => {
        return (
          <Link className={style.post} key={index} href={`/post/${post.id}`}>
            <h1>{post.title}</h1> 
            {post.file_id.length >= 1 &&
              <Image 
                src={CDN_URL + post.file_id}
                alt="Post Picture"
                sizes="100%"
                width={0}
                height={0}
              />
            }
            <span>{post.description.substring(0, 50)}</span>
          </Link>
        )
      })} 
    </div>
  );
}

export default UserPosts;
