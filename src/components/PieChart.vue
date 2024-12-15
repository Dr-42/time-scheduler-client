<template>
  <canvas ref="canvas"></canvas>
</template>

<script>
import { Chart, registerables } from "chart.js";
Chart.register(...registerables);

export default {
  props: {
    percentages: { type: Array, required: true },
  },
  mounted() {
    const ctx = this.$refs.canvas.getContext("2d");
    new Chart(ctx, {
      type: "pie",
      data: {
        labels: ["Work", "Exercise", "Leisure", "Sleep"],
        datasets: [
          {
            data: this.percentages,
            backgroundColor: ["#FF6384", "#36A2EB", "#FFCE56", "#4BC0C0"],
          },
        ],
      },
    });
  },
};
</script>
