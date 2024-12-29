<template>
  <div class="analysis-page">
    <h1>Analysis</h1>

    <!-- Date Range Selection -->
    <div class="date-selection">
      <label>
        Start Date:
        <input
          type="date"
          v-model="startDate"
          placeholder="DD-MM-YYYY"
        />
      </label>
      <label>
        End Date:
        <input
          type="date"
          v-model="endDate"
          placeholder="DD-MM-YYYY"
        />
      </label>
      <button :disabled="!canViewAnalysis" @click="fetchAnalysis">View Analysis</button>
    </div>
    <div v-if="error" class="error">
      <error-display :errorText="errorText" />
    </div>

    <!-- Validation Message -->
    <p v-if="!isDateRangeValid" class="error">Start date must be before end date</p>

    <!-- Analysis Charts -->
    <div v-if="analysisFetched" class="charts">
      <div class="pie-chart">
        <h2>Percentage Distribution</h2>
        <pie-chart :percentages="analysis.percentages" />
      </div>
      <div class="histogram">
        <h2>Trends</h2>
        <block-type-selector
          v-model:blocktypes="analysis.blocktypes"
          v-model:selectedBlockTypes="selectedBlockTypes"
        />
        <trend-chart
          v-model:trends="filteredTrends"
          v-model:blocktypes="analysis.blocktypes"
        />
      </div>
    </div>
  </div>
</template>

<script lang='ts'>
import PieChart from "../components/PieChart.vue";
import TrendChart from "../components/TrendChart.vue";
import BlockTypeSelector from "../components/BlockTypeSelector.vue";
import ErrorDisplay from "../components/ErrorDisplay.vue";

import { Analysis } from "../types";
import { invoke } from "@tauri-apps/api/core";

export default {
  components: {
    PieChart,
    TrendChart,
    BlockTypeSelector,
    ErrorDisplay
  },
  data() {
    return {
      startDate: null as null | string,
      endDate: null as null | string,
      selectedBlockTypes: [] as number[],
      analysis: null as null | Analysis,
      analysisFetched: false,
      error: false,
      errorText: "",
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
      if (!this.analysis) return [];
      return this.analysis.trends.filter((trend) =>
        this.selectedBlockTypes.includes(trend.blockTypeId)
      );
    },
  },
  methods: {
    async fetchAnalysis() {
      try {
        if (!this.startDate || !this.endDate) return;
        let start = new Date(this.startDate);
        let end = new Date(this.endDate);
        let data = await invoke("get_analysis", {
          startDate: start,
          endDate: end,
        });
        this.analysis = Analysis.fromJson(data);
        this.analysisFetched = true;
      } catch (error) {
        this.error = true;
        this.errorText = error as string;
      }
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
