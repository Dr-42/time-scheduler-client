<template>
  <div class="chart-container">
    <canvas ref="canvas"></canvas>
  </div>
</template>

<script>
import { Chart, registerables } from "chart.js";
import "chartjs-adapter-date-fns"; // Import the date adapter

Chart.register(...registerables);

export default {
  props: {
    trends: { type: Array, required: true },
    blocktypes: { type: Array, required: true },
  },
  mounted() {
    const ctx = this.$refs.canvas.getContext("2d");
    const blockColors = this.blocktypes.reduce((map, type) => {
      map[type.id] = type.color;
      return map;
    }, {});

    // Group trends by block type
    const datasets = this.trends.reduce((acc, trend) => {
      if (!acc[trend.block_type_id]) {
        acc[trend.block_type_id] = {
          label: this.blocktypes.find((bt) => bt.id === trend.block_type_id).name,
          backgroundColor: blockColors[trend.block_type_id],
          borderColor: blockColors[trend.block_type_id], // Use the same color for the line
          data: [],
          fill: false, // Ensures it’s just a line, no fill
        };
      }
      acc[trend.block_type_id].data.push({
        x: new Date(trend.day), // Ensure it's a Date object for time scale
        y: trend.time_spent,
      });
      return acc;
    }, {});

    // Create the chart
    new Chart(ctx, {
      type: "line",  // Line chart, perfect for frequency polygons
      data: {
        datasets: Object.values(datasets),
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
          x: {
            type: "time", // Use time scale
            time: {
              unit: "day", // Group by day
              tooltipFormat: 'll', // More readable tooltip format
            },
            min: this.trends.length > 0 ? new Date(this.trends[0].day - 1) : null,
            max: this.trends.length > 0 ? new Date(this.trends[this.trends.length - 1].day + 1) : null,
            title: {
              display: true,
              text: "Date",
            },
          },
          y: {
            beginAtZero: true,
            title: {
              display: true,
              text: "Time Spent (hours)",
            },
            min: 0,
            max: 24, // Adjust this based on your data range
          },
        },
      },
    });
  },
};
</script>

<style scoped>
.chart-container {
  position: relative;
  width: 100%;
  max-width: 800px;
  height: 400px;
  margin: 0 auto;
}
</style>
