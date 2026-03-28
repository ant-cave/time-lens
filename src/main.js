import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import '@primer/css/dist/marketing.css';
import '@primer/css/dist/primer.css';
import './styles/github-cards.css';

const app = createApp(App);
app.use(router);
app.mount("#app");
