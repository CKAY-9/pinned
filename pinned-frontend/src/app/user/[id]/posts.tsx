import { Post } from "@/api/post/dto";
import style from "./posts.module.scss";
import Link from "next/link";

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
            <p>{post.description.substring(0, 50)}</p>
          </Link>
        )
      })} 
    </div>
  );
}

export default UserPosts;
