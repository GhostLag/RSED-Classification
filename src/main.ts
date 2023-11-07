import "./styles.css";
// import App from "./App.svelte";
import Main from "./Main.svelte";

const app = new Main({
  target: document.getElementById("app"),
});

export default app;
