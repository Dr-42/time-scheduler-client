<template>
  <div class="chart-container">
    <canvas ref="canvas"></canvas>
  </div>
</template>

<script lang='ts'>
import { Chart, registerables } from "chart.js";
import "chartjs-adapter-date-fns"; // Import the date adapter
import { BlockType, Color, Trend } from "../types";

Chart.register(...registerables);

type ColorMap = { [key: number]: Color };
type Dataset = {
  label: string | undefined;
  backgroundColor: Color;
  borderColor: Color;
  data: { x: Date; y: number }[];
  fill: boolean;
};

type TrendMap = { [key: number]: Dataset };

export default {
  props: {
    trends: { type: Array<Trend>, required: true },
    blocktypes: { type: Array<BlockType>, default: [] },
  },
  data() {
    return {
      chart: null as Chart | null, // Store the Chart instance
    };
  },
  watch: {
    // Watch for changes in trends or blocktypes
    trends: {
      deep: true,
      handler() {
        this.updateChart();
      },
    },
    blocktypes: {
      deep: true,
      handler() {
        this.updateChart();
      },
    },
  },
  methods: {
    getYesterday(date: Date) {
      const yesterday = new Date(date);
      yesterday.setDate(yesterday.getDate() - 1);
      return yesterday;
    },
    getTomorrow(date: Date) {
      const tomorrow = new Date(date);
      tomorrow.setDate(tomorrow.getDate() + 1);
      return tomorrow;
    },
    initializeChart() {
      const canvas = this.$refs.canvas as HTMLCanvasElement;
      if (!canvas) {
        console.error("Canvas element not found");
        return;
      }
      const ctx = canvas.getContext("2d");

      const blockColors = this.blocktypes.reduce((map: ColorMap, type) => {
        map[type.id] = type.color;
        return map;
      }, {});

      // Group trends by block type
      const datasets = this.trends.reduce((acc: TrendMap, trend: Trend) => {
        if (!acc[trend.blockTypeId]) {
          acc[trend.blockTypeId] = {
            label: this.blocktypes.find((bt) => bt.id === trend.blockTypeId)?.name,
            backgroundColor: blockColors[trend.blockTypeId],
            borderColor: blockColors[trend.blockTypeId],
            data: [],
            fill: false,
          };
        }
        acc[trend.blockTypeId].data.push({
          x: new Date(trend.day), // Ensure it's a Date object for time scale
          y: trend.timeSpent.toHours(),
        });
        return acc;
      }, {});

      let chartXMin = this.trends.length > 0 ? this.getYesterday(new Date(this.trends[0].day)) : null;
      let chartXMax = this.trends.length > 0 ? this.getTomorrow(new Date(this.trends[this.trends.length - 1].day)) : null;

      if (chartXMin && chartXMax) {
        chartXMin.setHours(0, 0, 0, 0);
        chartXMax.setHours(0, 0, 0, 0);
        if (chartXMin.getDate() === chartXMax.getDate()) {
          chartXMin = this.getYesterday(chartXMin);
          chartXMax = this.getTomorrow(chartXMax);
        }
      } else {
        chartXMin = this.getYesterday(new Date());
        chartXMax = this.getTomorrow(new Date());
      }

      // Create the chart
      this.chart = new Chart(ctx, {
        type: "line", // Line chart
        data: {
          datasets: Object.values(datasets),
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          animation: false,
          scales: {
            x: {
              type: "time", // Use time scale
              time: {
                unit: "day", // Group by day
                tooltipFormat: "ll", // More readable tooltip format
              },
              min: chartXMin,
              max: chartXMax,
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
    updateChart() {
      // Destroy the existing chart instance if it exists
      if (this.chart) {
        this.chart.stop();
        this.chart.destroy();
      }
      // Reinitialize the chart with the updated data
      try {
        this.initializeChart();
      } catch (error) {
        console.error("Error updating chart:", error);
      }
    },
  },
  mounted() {
    this.initializeChart(); // Create the chart when the component is mounted
  },
  beforeDestroy() {
    if (this.chart) {
      this.chart.stop();
      this.chart.destroy(); // Clean up the chart instance when the component is destroyed
    }
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
