import { getCollection } from "@/api/collections/collections.client";
import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { redirect } from "next/navigation";
import CollectionClient from "./client";

const CollectionPage = async ({params}: {
  params: {
    id: string
  }
}) => {
  const collection_id = Number.parseInt(params.id);
  const collection = await getCollection(collection_id);
  if (collection === null) {
    redirect("/post/collection/explore");
  }

  const user = await getUserFromToken();

  return (
    <>
      <Header user={user} />
      <main className="container">
        <CollectionClient collection={collection} />
      </main>
      <Footer user={user} />
    </>
  );
}

export default CollectionPage;
