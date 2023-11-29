import { DISCORD_OAUTH_LINK, GITHUB_OAUTH_LINK } from "@/api/resources";
import Header from "@/components/header/header";
import Link from "next/link";
import style from "./login.module.scss";
import LoginClient from "./client";

const LoginPage = () => {
  return (
    <>
      <Header />
      <main className="container">
        <h1>User Login</h1>
        <div className={style.oauths}>
          {DISCORD_OAUTH_LINK !== undefined &&
            <Link className={style.oauth} style={{"backgroundColor": "#5865F2"}} href={DISCORD_OAUTH_LINK}>
              <span>Login with Discord</span>
            </Link>
          }
          {GITHUB_OAUTH_LINK !== undefined &&
            <Link className={style.oauth} style={{"backgroundColor": "#181818"}} href={GITHUB_OAUTH_LINK}>
              <span>Login with GitHub</span>
            </Link>
          }
        </div>
        <LoginClient />
      </main>
    </>
  );
}

export default LoginPage;
