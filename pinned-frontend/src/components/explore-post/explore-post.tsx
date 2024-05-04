import { CDN_URL } from "@/api/resources";
import Link from "next/link";
import LikeChip from "../like-chip/like-chip";
import UserChip from "../user-chip/user-chip";
import style from "./explore-post.module.scss";
import { Post } from "@/api/post/dto";

const ExplorePost = (props: { post: Post }) => {
  return (
    <Link
      href={`/post/${props.post.id}`}
      className={style.item}
      style={{
        background: `url(${CDN_URL + props.post.file_id})`,
        color: "white",
        backgroundSize: "cover",
        backgroundPositionX: "50%",
      }}
    >
      <div
        className={style.content}
        style={{
          display: "flex",
          flexDirection: "column",
          justifyContent: "space-between",
        }}
      >
        <section>
          <h1 style={{ fontSize: "4rem" }}>{props.post.title}</h1>
          <span>{props.post.description}</span>
        </section>
        <section style={{"display": "flex", "gap": "1rem", "justifyContent": "space-between"}}>
          <UserChip user_id={props.post.creator}></UserChip>
          <LikeChip
            likes={props.post.likes}
            dislikes={props.post.dislikes}
            post_id={props.post.id}
            post_type="post"
            user={null}
          ></LikeChip>
        </section>
      </div>
    </Link>
  );
};

export default ExplorePost;