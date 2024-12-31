<template>
  <div class="analysis-page">
    <h1>Analysis</h1>

    <!-- Date Range Selection -->
    <div class="date-selection">
      <div class="date-input">
        <div class="date-quali">Start Date</div>
        <date-picker
          :maxDate="new Date()"
          :minDate="new Date('2021-01-01')"
          :format="'DD-MM-YYYY'"
          @date-selected="startDate = $event"
        />
      </div>
      <div class="date-input">
        <div class="date-quali">End Date</div>
        <date-picker
          :maxDate="new Date()"
          :minDate="new Date('2021-01-01')"
          :format="'DD-MM-YYYY'"
          @date-selected="endDate = $event"
        />
      </div>
      <button :disabled="!canViewAnalysis" @click="fetchAnalysis">View Analysis</button>
    </div>
    <div v-if="error" class="error">
      <error-display :errorText="errorText" />
    </div>

    <!-- Validation Message -->
    <p v-if="!isDateRangeValid" class="error">Start date must be before end date</p>

    <!-- Analysis Charts -->
    <div v-if="analysisFetched && analysis" class="charts">
      <div class="pie-chart">
        <h2>Percentage Distribution</h2>
        <pie-chart
          v-model:percentages="analysis.percentages"
          v-model:blocktypes="analysis.blocktypes"
        />
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
import PieChart from "../components/charts/PieChart.vue";
import TrendChart from "../components/charts/TrendChart.vue";
import BlockTypeSelector from "../components/subviews/BlockTypeSelector.vue";
import ErrorDisplay from "../components/subviews/ErrorDisplay.vue";

import { Analysis } from "../types";
import { invoke } from "@tauri-apps/api/core";
import DatePicker from "../components/inputs/DatePicker.vue";

export default {
  components: {
    PieChart,
    TrendChart,
    BlockTypeSelector,
    ErrorDisplay,
    DatePicker
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
  color: white;
  background-color: var(--bg);
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

.date-input {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 10px;
}

.date-quali {
  font-size: 14px;
  margin-bottom: 5px;
}

button {
  font-size: 16px;
  padding: 10px 20px;
  background-color: var(--accent);
  color: #fff;
  border: none;
  border-radius: 5px;
  cursor: pointer;
}

button:disabled {
  background-color: var(--disabled-color);
  cursor: not-allowed;
}

button:hover:not(:disabled) {
  background-color: var(--accent2);
}
</style>
