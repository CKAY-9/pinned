import { createEffect, createSignal } from "solid-js";
import { Title } from "solid-start";
import { User } from "~/api/user/dto";

const UserProfile = () => {
  const [user, setUser] = createSignal<User | null>(null);
  const [loading, setLoading] = createSignal<boolean>(true);

  createEffect(async () => {
    setLoading(false);
  });

  if (loading()) {
    return (
      <>
        <Title>Loading // Pinned</Title>
      </>
    );
  }

  if (user() === null) {
    return (
      <>
        <Title>Not Found // Pinned</Title>
      </>
    );
  }

  return (
    <>
      <Title>Pinned</Title>
    </>
  );
}

export default UserProfile;
