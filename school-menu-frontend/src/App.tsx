import { Component, createSignal, Show } from "solid-js";
import { Admin } from "./Admin";
import { User } from "./User";

const App: Component = () => {
  const [signedIn, setSignedIn] = createSignal(false);
  const [user, setUser] = createSignal();
  return (
    <>
      <Show
        when={!signedIn()}
        fallback={
          <>
            <Show when={user()} fallback={<Admin />}>
              <User />
            </Show>
          </>
        }
      >
        <div>
          <button
            onClick={() => {
              setSignedIn(true);
              setUser(false);
            }}
          >
            Admin
          </button>
          <button
            onClick={() => {
              setSignedIn(true);
              setUser(true);
            }}
          >
            User
          </button>
        </div>
      </Show>
    </>
  );
};

export default App;
