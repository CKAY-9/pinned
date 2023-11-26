import { Title } from "solid-start";
import Header from "~/components/header/header";
import { DISCORD_OAUTH, GITHUB_OAUTH } from "~/utils/resources";
import "./auth.css";

const AuthPage = () => {
  return (
    <>
      <Title>Login // Pinned</Title>
      <Header></Header> 
      <main class="container">
        <h1>User Login</h1>
        <div class="links">
          <a href={DISCORD_OAUTH} class="link" id="discord">
            <img src="/discord-mark-white.svg" alt="Discord" />
            <span>Login using Discord</span>
          </a>
          <a href={GITHUB_OAUTH} class="link" id="github">
            <img src="/github-mark-white.svg" alt="GitHub" />
            <span>Login using GitHub</span>
          </a>
        </div>
      </main>
    </>
  );
}

export default AuthPage;
