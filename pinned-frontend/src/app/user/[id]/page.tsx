import { getUserFromID, getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import style from "./user.module.scss";
import Image from "next/image";
import { redirect } from "next/navigation";

const UserPage = async ({params}: {
  params: {
    id: string
  }
}) => {
  const user_id = Number.parseInt(params.id);
  const user = await getUserFromID(user_id);

  if (user === null) {
    redirect("/user/search");
  }

  const self_user = await getUserFromToken();
  const is_self = (self_user === null ? 0 : self_user.id) === user.id;

  return (
    <>
      <Header user={self_user} />
      <main className="container">
        <div className={style.user_header}>
          <Image src={user.avatar} alt="Profile Picture" sizes="100%" width={0} height={0} className={style.pfp} />
          <h1>{user.username}</h1>
        </div>
      </main>
      <Footer user={self_user} />
    </>
  );
}

export default UserPage;
