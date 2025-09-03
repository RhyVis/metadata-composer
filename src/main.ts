import 'quasar/src/css/index.sass';
import '@quasar/extras/material-icons/material-icons.css';
import '@quasar/extras/fontawesome-v6/fontawesome-v6.css';
import '@/assets/main.scss';
import { Dialog, Loading, Notify, Quasar } from 'quasar';
import quasarLang from 'quasar/lang/zh-CN';
import { createApp } from 'vue';
import { initKeyBlocker, initLogger } from '@/api/hook.ts';
import App from '@/App.vue';
import { initTrayIcon } from '@/hooks/useTray';
import i18n from '@/i18n';
import router from '@/router';
import pinia from '@/stores';

initLogger();
initKeyBlocker();
initTrayIcon();

console.info('Creating frontend application...');

const app = createApp(App);

app.use(pinia);
app.use(router);
app.use(i18n);

app.use(Quasar, {
  plugins: {
    Notify,
    Dialog,
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
