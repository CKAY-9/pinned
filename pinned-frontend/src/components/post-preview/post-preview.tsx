import Link from "next/link";
import Image from "next/image";
import style from "./post-preview.module.scss";
import { Post } from "@/api/post/dto";
import { CDN_URL } from "@/api/resources";

const PostPreview = (props: { post: Post; pinned?: boolean }) => {
  return (
    <>
      <Link className={style.post} href={`/post/${props.post.id}`}>
        <section className={style.info}>
          <h1>{props.post.title}</h1>
          {props.pinned && (
            <Image
              src="/icons/star.svg"
              alt="Pinned"
              sizes="100%"
              width={36}
              height={36}
            />
          )}
        </section>
        {props.post.file_id.length >= 1 && (
          <Image
            src={CDN_URL + props.post.file_id}
            className={style.preview}
            alt="Post Picture"
            sizes="100%"
            width={0}
            height={0}
          />
        )}
        <span>{props.post.description.substring(0, 50)}</span>
      </Link>
    </>
  );
};

export default PostPreview;
