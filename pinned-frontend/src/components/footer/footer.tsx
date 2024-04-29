import Link from "next/link";
import style from "./footer.module.scss";
import { User } from "@/api/user/dto";
import Image from "next/image";

const Footer = (props: {
  user: User | null
}) => {
  return (
    <footer id="footer_main" className={style.footer}>
      <section>
        <div className={style.name}>
          <Image 
            src="/marks/pinned-mark-white.png"
            alt="Pinned"
            sizes="100%"
            width={0}
            height={0}
            className={style.logo}
          />
          <h1>Pinned</h1>
        </div>
        <span>Made with ❤️ by <Link href="/team">the team</Link></span>
        <div className={style.links}>
          <Link href="https://github.com/CKAY-9/pinned">
            <Image src="/marks/github-mark-white.svg" alt="GitHub" sizes="100%" width={0} height={0} />
          </Link>
        </div>
      </section>
      <section>
        <strong>General</strong>
        <Link href="/">Home</Link> 
        <Link href="/about">About</Link> 
        <Link href="/team">Our Team</Link> 
      </section>
      <section>
        <strong>Posts</strong>
        <Link href="/post/explore">Explore</Link> 
        <Link href="/post/search">Search</Link> 
        {props.user !== null && <Link href="/post/new">New Post</Link>}
      </section>
      <section>
        <strong>Collections</strong>
        <Link href="/post/collection/explore">Explore</Link> 
        <Link href="/post/collection/search">Search</Link> 
        {props.user !== null && <Link href="/post/collection/new">New Collection</Link>}
      </section>
      <section>
        <strong>Users</strong>
        <Link href="/user/search">Search</Link> 
        <Link href="/user/explore">Explore</Link>
        {props.user === null
          ? <Link href="/user/login">Login</Link>
          : <Link href={`/user/${props.user.id}`}>My Profile</Link>
        }
      </section>
    </footer>
  );
}

export default Footer;
