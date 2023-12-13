import { Post } from "@/api/post/dto"
import { User } from "@/api/user/dto"
import style from "./post.module.scss";
import Image from "next/image";

const PostClient = (props: {
  post: Post,
  user: User | null
}) => {
  const is_owner = props.user?.id == props.post.creator;
    
  return (
    <>
      <div className={style.post_container}>
        <h1>{props.post.title}</h1>
        <section className={style.interaction}>
          <button>
            <Image
              src="/icons/like.svg"
              alt="Likes"
              sizes="100%"
              width={0}
              height={0}
              className={style.like}
              style={{"opacity": props.post.likes.includes(props.user?.id || 0) ? 1 : 0.5}}
            />
            <span>{props.post.likes.length}</span>
          </button>
          <button>
            <Image
              src="/icons/like.svg"
              alt="Likes"
              sizes="100%"
              width={0}
              height={0}
              className={style.like}
              style={{"opacity": props.post.likes.includes(props.user?.id || 0) ? 1 : 0.5}}
            />
            <span>{props.post.dislikes.length}</span>
          </button>
        </section>
        <p>{props.post.description}</p>
      </div>
    </>
  );
}

export default PostClient;
