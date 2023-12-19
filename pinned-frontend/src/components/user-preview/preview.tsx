import { User } from "@/api/user/dto"
import style from "./preview.module.scss"
import Image from "next/image"

const UserPreview = (props: {
  user: User
}) => {
  return (
    <div className={style.preview}>
      <Image 
        src={props.user.avatar}
        alt="PFP"
        sizes="100%"
        width={0}
        height={0}
        className={style.icon}
      />
      <span className={style.name}>{props.user.username}</span>
      <span className={style.id}>{props.user.id}</span>
    </div>
  );
}

export default UserPreview;
