import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";

export const generateMetadata = (): Metadata => {
  return {
    "title": "Search Collections // Pinned",
    "description": "Search all public collections on Pinned."
  }
}

const CollectionSearchPage = async () => {
  const user = await getUserFromToken();

  return (
    <>
      <Header user={user} />
      <main className="container">
        <h1>Search Collections</h1>
      </main>
      <Footer user={user} />
    </>
  );
}

export default CollectionSearchPage;
