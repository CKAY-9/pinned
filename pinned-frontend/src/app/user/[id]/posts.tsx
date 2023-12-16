import { Post } from "@/api/post/dto";
import style from "@/components/post-preview/post-preview.module.scss";
import PostPreview from "@/components/post-preview/post-preview";

const UserPosts = (props: {
  posts: Post[] | null
}) => {
  if (props.posts === null) return;

  return (
    <div className={style.posts}>
      {props.posts.map((post: Post, index: number) => {
        return (
          <PostPreview key={index} post={post} />
        )
      })} 
    </div>
  );
}

export default UserPosts;
