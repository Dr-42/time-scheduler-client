<template>
  <div class="analysis-page">
    <h1>Analysis</h1>

    <!-- Date Range Selection -->
    <div class="date-selection">
      <label>
        Start Date:
        <input type="date" v-model="startDate" />
      </label>
      <label>
        End Date:
        <input type="date" v-model="endDate" />
      </label>
      <button :disabled="!canViewAnalysis" @click="fetchAnalysis">View Analysis</button>
    </div>

    <!-- Validation Message -->
    <p v-if="!isDateRangeValid" class="error">Start date must be before end date</p>

    <!-- Analysis Charts -->
    <div v-if="analysisFetched" class="charts">
      <div class="pie-chart">
        <h2>Percentage Distribution</h2>
        <pie-chart :percentages="dummyAnalysis.percentages" />
      </div>
      <div class="histogram">
        <h2>Trends</h2>
        <block-type-selector
          :blocktypes="dummyBlockTypes"
          v-model:selectedBlockTypes="selectedBlockTypes"
        />
        <trend-chart :trends="filteredTrends" :blocktypes="dummyBlockTypes" />
      </div>
    </div>
  </div>
</template>

<script>
import PieChart from "../components/PieChart.vue";
import TrendChart from "../components/TrendChart.vue";
import BlockTypeSelector from "../components/BlockTypeSelector.vue";

export default {
  components: {
    PieChart,
    TrendChart,
    BlockTypeSelector,
  },
  data() {
    return {
      startDate: null,
      endDate: null,
      selectedBlockTypes: [],
      dummyAnalysis: {
        percentages: [0.4, 0.3, 0.2, 0.1], // Example: 40%, 30%, etc.
        trends: [
          { day: "2024-12-11", block_type_id: 1, time_spent: 5 },
          { day: "2024-12-11", block_type_id: 2, time_spent: 2 },
          { day: "2024-12-11", block_type_id: 3, time_spent: 8 },
          { day: "2024-12-12", block_type_id: 1, time_spent: 2 },
          { day: "2024-12-12", block_type_id: 2, time_spent: 4 },
          { day: "2024-12-12", block_type_id: 3, time_spent: 4 },
          { day: "2024-12-13", block_type_id: 1, time_spent: 3 },
          { day: "2024-12-13", block_type_id: 2, time_spent: 6 },
          { day: "2024-12-13", block_type_id: 3, time_spent: 9 },
        ],
      },
      dummyBlockTypes: [
        { id: 1, name: "Work", color: "#FF6384" },
        { id: 2, name: "Exercise", color: "#36A2EB" },
        { id: 3, name: "Leisure", color: "#FFCE56" },
        { id: 4, name: "Sleep", color: "#4BC0C0" },
      ],
      analysisFetched: false,
    };
  },
  computed: {
    canViewAnalysis() {
      return this.startDate && this.endDate && this.isDateRangeValid;
    },
    isDateRangeValid() {
      return this.startDate && this.endDate && new Date(this.startDate) <= new Date(this.endDate);
    },
    filteredTrends() {
      return this.dummyAnalysis.trends.filter((trend) =>
        this.selectedBlockTypes.includes(trend.block_type_id)
      );
    },
  },
  methods: {
    fetchAnalysis() {
      // Simulate fetching data (real server hookup later)
      this.analysisFetched = true;
    },
  },
};
</script>

<style scoped>
.analysis-page {
  max-width: 800px;
  margin: 0 auto;
  text-align: center;
}
.date-selection {
  margin-bottom: 20px;
}
.error {
  color: red;
}
.charts {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.pie-chart,
.histogram {
  margin: 20px 0;
}
</style>
