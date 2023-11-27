import Link from "next/link";
import style from "./header.module.scss";

const Header = () => {
  return (
    <header className={style.header}>
      <section>
        <Link href="/">LOGO</Link>
      </section>
      <section>
        <Link href="/user/login">Login</Link>
      </section>
    </header>
  );
}

export default Header;
