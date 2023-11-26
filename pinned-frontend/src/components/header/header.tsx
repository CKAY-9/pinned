import "./header.css";

const Header = () => {
  return (
    <header>
      <section>
        <a href="/">
          <h1>LOGO</h1>
        </a>
      </section>
      <section>
        <a href="/user/auth">Login/Register</a>
      </section>
    </header>
  );
}

export default Header;
