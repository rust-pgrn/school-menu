import { Component, createSignal, Show } from "solid-js";
import { Admin } from "./Admin";
import { User } from "./User";

const App: Component = () => {
  const [signedIn, setSignedIn] = createSignal(false);
  const [isUser, setIsUser] = createSignal();
  const [user, setUser] = createSignal();
  return (
    <>
      <Show
        when={!signedIn()}
        fallback={
          <>
            <Show when={isUser()} fallback={<Admin />}>
              <User />
            </Show>
          </>
        }
      >
        <form
          onInput={(event) => {
            event.preventDefault();
            setSignedIn(true);
            console.log(event.target.nodeValue);
          }}
        >
          <label for="students">Who Are You?</label>
          <select id="students" name="students" size="4">
            <option value="Mohamed Ali">Mohamed Ali</option>
            <option value="Karim Tamer">Karim Tamer</option>
            <option value="Amina Kenzi">Amina Kenzi</option>
            <option value="Marcus Higenyi">Marcus Higenyi</option>
            <option value="Sherif Khalifa">Sherif Khalifa</option>
            <option
              value="Ms. Aline"
              onClick={() => {
                setIsUser(true);
              }}
            >
              Ms. Aline
            </option>
          </select>
        </form>
      </Show>
    </>
  );
};

export default App;
