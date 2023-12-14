import { User } from "@/api/user/dto";
import style from "./home.module.scss";

const HomeClient = (props: {
  user: User | null
}) => {
  return (
    <>
      <h1 className={style.pinned}>Today's Pinned</h1>
    </>
  );
}

export default HomeClient;
