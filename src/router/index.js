// router/index.js
import { createMemoryHistory, createRouter } from 'vue-router'
import Home from '../views/Home.vue';
import History from '../views/History.vue';
import Analysis from '../views/Analysis.vue';

const routes = [
	{ path: '/', name: 'Home', component: Home },
	{ path: '/history', name: 'History', component: History },
	{ path: '/analysis', name: 'Analysis', component: Analysis },
];

const router = createRouter({
	history: createMemoryHistory(),
	routes,
});

export default router;
