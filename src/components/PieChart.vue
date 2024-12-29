<template>
  <canvas ref="canvas"></canvas>
</template>

<script lang="ts">
import { Chart, registerables } from "chart.js";
import { BlockType } from "../types";
Chart.register(...registerables);

export default {
  props: {
    percentages: { type: Array<number>, default: [] },
    blocktypes: { type: Array<BlockType>, default: [] },
  },
  data() {
    return {
      chart: null as Chart | null,
    };
  },
  watch: {
    percentages: {
      deep: true,
      handler() {
        this.updateChart();
      },
    },
  },
  methods: {
    initializeChart() {
      const canvas = this.$refs.canvas as HTMLCanvasElement;
      const ctx = canvas.getContext("2d");

      let labels = this.blocktypes.map((blocktype) => blocktype.name);
      let colors = this.blocktypes.map((blocktype) => blocktype.color);

      // Create the pie chart
      this.chart = new Chart(ctx, {
        type: "pie",
        data: {
          labels: labels,
          datasets: [
            {
              data: this.percentages,
              backgroundColor: colors,
            },
          ],
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: {
            legend: {
              display: true,
              position: "bottom",
            },
            // Convert the float numbers to percentages on hover
            tooltip: {
              callbacks: {
                label: (context) => {
                  let label = context.label || "";
                  if (label) {
                    label += ": ";
                  }
                  if (context.parsed) {
                    label += `${(context.parsed * 100).toFixed(2)}%`;
                  }
                  return label;
                },
              },
            },
          },
        },
      });
    },
    updateChart() {
      if (this.chart) {
        this.chart.destroy();
      }
      this.initializeChart(); // Reinitialize with updated data
    },
  },
  mounted() {
    this.initializeChart(); // Create the chart when the component is mounted
  },
  beforeDestroy() {
    if (this.chart) {
      this.chart.destroy(); // Clean up the chart instance when the component is destroyed
    }
  },
};
</script>

<style scoped>
canvas {
  max-width: 400px;
  max-height: 400px;
  margin: 0 auto;
}
</style>
