import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import HomeClient from "./client";

const Index = async () => {
  const user = await getUserFromToken();

  return (
    <>
      <Header user={user} />
      <main className="container">
        <HomeClient user={user} />
      </main>
      <Footer user={user} />
    </>
  );
}

export default Index;
