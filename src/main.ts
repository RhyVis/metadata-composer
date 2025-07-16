import 'quasar/src/css/index.sass';
import '@quasar/extras/material-icons/material-icons.css';
import '@/assets/main.scss';
import { Loading, Notify, Quasar } from 'quasar';
import quasarLang from 'quasar/lang/zh-CN';
import { createApp } from 'vue';
import { initLogger } from '@/api/hook.ts';
import App from '@/App.vue';
import router from '@/router';
import pinia from '@/stores';

initLogger();

console.info('Creating frontend application...');

const app = createApp(App);

app.use(pinia);
app.use(router);

app.use(Quasar, {
  plugins: {
    Notify,
    Loading,
  },
  lang: quasarLang,
  config: {
    dark: 'auto',
  },
});

app.mount('#app');

setTimeout(() => {
  const loadingElement = document.getElementById('app-loading');
  if (loadingElement) {
    loadingElement.style.opacity = '0';
    loadingElement.style.transition = 'opacity 0.5s ease';

    setTimeout(() => {
      loadingElement.remove();
    }, 500);
  }
}, 100);
