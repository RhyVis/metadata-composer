import { createPinia } from 'pinia';
import { TauriPluginPinia } from '@tauri-store/pinia';

const pinia = createPinia();
pinia.use(TauriPluginPinia());

export default pinia;
