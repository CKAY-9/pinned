import Link from "next/link";
import style from "./header.module.scss";
import { User } from "@/api/user/dto";

const Header = (props: {
  user: User | null
}) => {
  return (
    <header className={style.header}>
      <section>
        <Link href="/">LOGO</Link>
      </section>
      <section>
        {props.user === null
          ? <Link href="/user/login">Login</Link>
          : <Link href={`/user/${props.user.id}`}>My Profile</Link>
        }
      </section>
    </header>
  );
}

export default Header;
