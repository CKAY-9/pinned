import { getUserFromID } from "@/api/user/user.client";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import style from "./user.module.scss";
import Image from "next/image";
import { redirect } from "next/navigation";
import UserCreations from "./client";
import { Metadata } from "next";
import { getUserFromToken } from "@/api/user/user";

export const generateMetadata = async ({params}: {
  params: {
    id: string
  }
}): Promise<Metadata> => {
  const user_id = Number.parseInt(params.id);
  const user = await getUserFromID(user_id);
  if (user === null) {
    return {
      "title": "User Profile // Pinned",
      "description": "Find users on Pinned."
    }
  }

  return {
    "title": `${user.username}'s Profile // Pinned`,
    "description": `View ${user.username}'s Profile on Pinned. Bio: ${user.bio}`
  }
}

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

  return (
    <>
      <Header user={self_user} />
      <main className="container">
        <div className={style.user_header}>
          <Image src={user.avatar} alt="Profile Picture" sizes="100%" width={0} height={0} className={style.pfp} />
          <div>
            <h1>{user.username}</h1>
            <span>{user.bio}</span>
          </div>
        </div>
        <UserCreations user={self_user} profile={user} />
      </main>
      <Footer user={self_user} />
    </>
  );
}

export default UserPage;
