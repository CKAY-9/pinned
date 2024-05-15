import Link from "next/link";
import LikeChip from "../like-chip/like-chip";
import UserChip from "../user-chip/user-chip";
import style from "./explore-post.module.scss";
import { Collection } from "@/api/collections/dto";

const ExploreCollection = (props: { collection: Collection }) => {
  return (
    <Link
      href={`/post/collection/${props.collection.id}`}
      className={style.item}
      style={{
        backgroundColor: "var(--primary)",
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
          opacity: "1",
          "background": "transparent"
        }}
      >
        <section>
          <h1 style={{ fontSize: "3rem" }}>{props.collection.name}</h1>
          <span>{props.collection.description}</span>
        </section>
        <section style={{"display": "flex", "gap": "1rem", "justifyContent": "space-between"}}>
          <UserChip user_id={props.collection.creator}></UserChip>
          <LikeChip
            likes={props.collection.likes}
            dislikes={props.collection.dislikes}
            post_id={props.collection.id}
            post_type="post"
            user={null}
          ></LikeChip>
        </section>
      </div>
    </Link>
  );
};

export default ExploreCollection;