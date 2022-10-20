import { createSignal, Show } from "solid-js";

export function User() {
  const [choseId, setChoseId] = createSignal(false);
  return (
    <>
      <Show
        when={choseId()}
        fallback={
          <>
            <form
              onSubmit={(event) => {
                event.preventDefault();
                setChoseId(true);
              }}
            >
              <label for="students">Who Are You?</label>
              <input list="students" id="browser" />
              <datalist id="students">
                <option value="Mohamed Ali" />
                <option value="Karim Tamer" />
                <option value="Amina Kenzi" />
                <option value="Marcus Higenyi" />
                <option value="Sherif Khalifa" />
              </datalist>
            </form>
          </>
        }
      >
        Menu For Students
      </Show>
    </>
  );
}
