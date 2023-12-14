import { DISCORD_OAUTH_LINK, GITHUB_OAUTH_LINK } from "@/api/resources";
import Header from "@/components/header/header";
import Link from "next/link";
import style from "./login.module.scss";
import LoginClient from "./client";
import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Image from "next/image";
import { Metadata } from "next";

export const generateMetadata = (): Metadata => {
  return {
    "title": "Login to Pinned",
    "description": "Login to Pinned using your GitHub, and Discord accounts."
  }
}

const LoginPage = async () => {
  const user = await getUserFromToken();

  return (
    <>
      <Header user={user} />
      <main className="container">
        <h1>User Login</h1>
        <div className={style.oauths}>
          {DISCORD_OAUTH_LINK !== undefined &&
            <Link className={style.oauth} style={{"backgroundColor": "#5865F2"}} href={DISCORD_OAUTH_LINK}>
              <Image
                src="/marks/discord-mark-white.svg"
                alt="Discord"
                sizes="100%"
                width={0}
                height={0}
              />
              <span>Login with Discord</span>
            </Link>
          }
          {GITHUB_OAUTH_LINK !== undefined &&
            <Link className={style.oauth} style={{"backgroundColor": "#181818"}} href={GITHUB_OAUTH_LINK}>
              <Image
                src="/marks/github-mark-white.svg"
                alt="GitHub"
                sizes="100%"
                width={0}
                height={0}
              />
              <span>Login with GitHub</span>
            </Link>
          }
        </div>
        <LoginClient />
      </main>
      <Footer user={user} />
    </>
  );
}

export default LoginPage;
