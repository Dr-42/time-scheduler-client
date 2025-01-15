import { createMemoryHistory, createRouter, Router } from "vue-router";
import Home from "../views/Home.vue";
import History from "../views/History.vue";
import Analysis from "../views/Analysis.vue";
import HistoryTimeCards from "../views/HistoryTimeCards.vue";
import TestBed from "../components/subviews/TestBed.vue";

const routes = [
  { path: "/", name: "Home", component: Home },
  { path: "/history", name: "History", component: History },
  {
    path: "/timecards/:date",
    name: "HistoryTimeCards",
    component: HistoryTimeCards,
    props: true,
  },
  { path: "/analysis", name: "Analysis", component: Analysis },
];

const router: Router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
